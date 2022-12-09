use super::JVM;

use crate::access_macros;
use crate::class::Class;
use crate::constant_pool::Entry;
use crate::errorcodes::{Error, Opcode};
use crate::reference::{Reference, Monitor};
use crate::value::{Value, VarValue};

use std::any::Any;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use dyn_clone::*;
#[cfg(not(target_family = "wasm"))]
use {
    inkwell::basic_block::BasicBlock,
    inkwell::builder::Builder,
    inkwell::context::Context,
    inkwell::execution_engine::{ExecutionEngine, JitFunction},
    inkwell::module::Module,
    inkwell::values::{FunctionValue, PointerValue, IntValue},

};

pub mod constants;
pub mod loads;
pub mod stores;
pub mod stack;
pub mod math;
pub mod conversions;
pub mod comparisons;
pub mod control;
pub mod references;
pub mod extended;
pub mod reserved;

// FIXME: Test code address translation
#[macro_export]
macro_rules! compress_addr {
    ($addr:ident) => {
        fn compress_range(&mut self, this_pc: usize, translation_map: &HashMap<usize, usize>) {
            let goal_addr = this_pc as isize + self.$addr;
            self.$addr = (*translation_map.get(&(goal_addr as usize)).unwrap() as isize - *translation_map.get(&this_pc).unwrap() as isize);
        }
    };
}

pub trait Instruction : core::fmt::Debug + DynClone {
    fn name(&self) -> &'static str;
    fn new(_v: &mut Vec<u8>, _cpool: &Vec<Entry>, _jvm: &mut JVM, _was_wide: bool, _true_pc: usize) -> Result<Self, Error> where Self : Sized;
    #[inline] fn execute(&mut self, _ : &mut JVM) -> Result<(), Error> {
        panic!("TODO execution not implemented for {}", self.name());
    }
    fn compress_range(&mut self, _this_pc: usize, _translation_map: &HashMap<usize, usize>) {}
    fn as_any(&self) -> &dyn Any;
    fn eq(&self, other: &dyn Instruction) -> bool;

    fn can_jit(&self) -> bool { false }
    // A very basic jit outline, without support for control flow. 
    #[cfg(not(target_family = "wasm"))]
    fn jit(&self, context: &'static Context, module: &Module<'static>, builder: &Builder<'static>, 
            engine: &ExecutionEngine<'static>, name: &String, func: FunctionValue, 
            locals: &Vec<PointerValue>, blocks: &HashMap<usize, BasicBlock>, stack: &PointerValue, top: &PointerValue) {
        panic!("Attempted to jit a non-jitable instruction");
    }
    
    fn is_control_flow(&self) -> bool { false }
}

impl std::fmt::Display for dyn Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
    }
}
impl PartialEq for dyn Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

clone_trait_object!(Instruction);

pub fn new_instruction(v: &mut Vec<u8>, c: &Vec<Entry>, jvm: &mut JVM, was_wide: bool, true_pc: usize) -> Result<Box<dyn Instruction>, Error> {
    let op = v[0];
    // println!("Creating op {}", op);
    v.remove(0);
    match op {
        0 => Ok(Box::new(constants::Nop::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        1 => Ok(Box::new(constants::AConstNull::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        2 => Ok(Box::new(constants::IConstM1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        3 => Ok(Box::new(constants::IConst0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        4 => Ok(Box::new(constants::IConst1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        5 => Ok(Box::new(constants::IConst2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        6 => Ok(Box::new(constants::IConst3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        7 => Ok(Box::new(constants::IConst4::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        8 => Ok(Box::new(constants::IConst5::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        9 => Ok(Box::new(constants::LConst0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        10 => Ok(Box::new(constants::LConst1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        11 => Ok(Box::new(constants::FConst0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        12 => Ok(Box::new(constants::FConst1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        13 => Ok(Box::new(constants::FConst2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        14 => Ok(Box::new(constants::DConst0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        15 => Ok(Box::new(constants::DConst1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        16 => Ok(Box::new(constants::BiPush::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        17 => Ok(Box::new(constants::SiPush::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        18 => Ok(Box::new(constants::Ldc::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        19 => Ok(Box::new(constants::LdcW::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        20 => Ok(Box::new(constants::Ldc2W::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        21 => Ok(Box::new(loads::ILoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        22 => Ok(Box::new(loads::LLoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        23 => Ok(Box::new(loads::FLoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        24 => Ok(Box::new(loads::DLoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        25 => Ok(Box::new(loads::ALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        26 => Ok(Box::new(loads::ILoad0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        27 => Ok(Box::new(loads::ILoad1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        28 => Ok(Box::new(loads::ILoad2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        29 => Ok(Box::new(loads::ILoad3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        30 => Ok(Box::new(loads::LLoad0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        31 => Ok(Box::new(loads::LLoad1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        32 => Ok(Box::new(loads::LLoad2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        33 => Ok(Box::new(loads::LLoad3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        34 => Ok(Box::new(loads::FLoad0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        35 => Ok(Box::new(loads::FLoad1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        36 => Ok(Box::new(loads::FLoad2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        37 => Ok(Box::new(loads::FLoad3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        38 => Ok(Box::new(loads::DLoad0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        39 => Ok(Box::new(loads::DLoad1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        40 => Ok(Box::new(loads::DLoad2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        41 => Ok(Box::new(loads::DLoad3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        42 => Ok(Box::new(loads::ALoad0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        43 => Ok(Box::new(loads::ALoad1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        44 => Ok(Box::new(loads::ALoad2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        45 => Ok(Box::new(loads::ALoad3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        46 => Ok(Box::new(loads::IALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        47 => Ok(Box::new(loads::LALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        48 => Ok(Box::new(loads::FALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        49 => Ok(Box::new(loads::DALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        50 => Ok(Box::new(loads::AALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        51 => Ok(Box::new(loads::BALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        52 => Ok(Box::new(loads::CALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        53 => Ok(Box::new(loads::SALoad::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        54 => Ok(Box::new(stores::IStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        55 => Ok(Box::new(stores::LStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        56 => Ok(Box::new(stores::FStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        57 => Ok(Box::new(stores::DStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        58 => Ok(Box::new(stores::AStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        59 => Ok(Box::new(stores::IStore0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        60 => Ok(Box::new(stores::IStore1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        61 => Ok(Box::new(stores::IStore2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        62 => Ok(Box::new(stores::IStore3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        63 => Ok(Box::new(stores::LStore0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        64 => Ok(Box::new(stores::LStore1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        65 => Ok(Box::new(stores::LStore2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        66 => Ok(Box::new(stores::LStore3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        67 => Ok(Box::new(stores::FStore0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        68 => Ok(Box::new(stores::FStore1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        69 => Ok(Box::new(stores::FStore2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        70 => Ok(Box::new(stores::FStore3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        71 => Ok(Box::new(stores::DStore0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        72 => Ok(Box::new(stores::DStore1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        73 => Ok(Box::new(stores::DStore2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        74 => Ok(Box::new(stores::DStore3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        75 => Ok(Box::new(stores::AStore0::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        76 => Ok(Box::new(stores::AStore1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        77 => Ok(Box::new(stores::AStore2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        78 => Ok(Box::new(stores::AStore3::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        79 => Ok(Box::new(stores::IAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        80 => Ok(Box::new(stores::LAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        81 => Ok(Box::new(stores::FAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        82 => Ok(Box::new(stores::DAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        83 => Ok(Box::new(stores::AAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        84 => Ok(Box::new(stores::BAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        85 => Ok(Box::new(stores::CAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        86 => Ok(Box::new(stores::SAStore::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        87 => Ok(Box::new(stack::Pop::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        88 => Ok(Box::new(stack::Pop2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        89 => Ok(Box::new(stack::Dup::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        90 => Ok(Box::new(stack::DupX1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        91 => Ok(Box::new(stack::DupX2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        92 => Ok(Box::new(stack::Dup2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        93 => Ok(Box::new(stack::Dup2X1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        94 => Ok(Box::new(stack::Dup2X2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        95 => Ok(Box::new(stack::Swap::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        96 => Ok(Box::new(math::IAdd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        97 => Ok(Box::new(math::LAdd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        98 => Ok(Box::new(math::FAdd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        99 => Ok(Box::new(math::DAdd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        100 => Ok(Box::new(math::ISub::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        101 => Ok(Box::new(math::LSub::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        102 => Ok(Box::new(math::FSub::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        103 => Ok(Box::new(math::DSub::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        104 => Ok(Box::new(math::IMul::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        105 => Ok(Box::new(math::LMul::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        106 => Ok(Box::new(math::FMul::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        107 => Ok(Box::new(math::DMul::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        108 => Ok(Box::new(math::IDiv::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        109 => Ok(Box::new(math::LDiv::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        110 => Ok(Box::new(math::FDiv::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        111 => Ok(Box::new(math::DDiv::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        112 => Ok(Box::new(math::IRem::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        113 => Ok(Box::new(math::LRem::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        114 => Ok(Box::new(math::FRem::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        115 => Ok(Box::new(math::DRem::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        116 => Ok(Box::new(math::INeg::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        117 => Ok(Box::new(math::LNeg::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        118 => Ok(Box::new(math::FNeg::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        119 => Ok(Box::new(math::DNeg::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        120 => Ok(Box::new(math::IShl::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        121 => Ok(Box::new(math::LShl::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        122 => Ok(Box::new(math::IShr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        123 => Ok(Box::new(math::LShr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        124 => Ok(Box::new(math::IUshr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        125 => Ok(Box::new(math::LUshr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        126 => Ok(Box::new(math::IAnd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        127 => Ok(Box::new(math::LAnd::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        128 => Ok(Box::new(math::IOr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        129 => Ok(Box::new(math::LOr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        130 => Ok(Box::new(math::IXor::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        131 => Ok(Box::new(math::LXor::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        132 => Ok(Box::new(math::IInc::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        133 => Ok(Box::new(conversions::I2L::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        134 => Ok(Box::new(conversions::I2F::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        135 => Ok(Box::new(conversions::I2D::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        136 => Ok(Box::new(conversions::L2I::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        137 => Ok(Box::new(conversions::L2F::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        138 => Ok(Box::new(conversions::L2D::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        139 => Ok(Box::new(conversions::F2I::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        140 => Ok(Box::new(conversions::F2L::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        141 => Ok(Box::new(conversions::F2D::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        142 => Ok(Box::new(conversions::D2I::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        143 => Ok(Box::new(conversions::D2L::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        144 => Ok(Box::new(conversions::D2F::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        145 => Ok(Box::new(conversions::I2B::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        146 => Ok(Box::new(conversions::I2C::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        147 => Ok(Box::new(conversions::I2S::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        148 => Ok(Box::new(comparisons::LCmp::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        149 => Ok(Box::new(comparisons::FCmpL::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        150 => Ok(Box::new(comparisons::FCmpG::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        151 => Ok(Box::new(comparisons::DCmpL::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        152 => Ok(Box::new(comparisons::DcmpG::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        153 => Ok(Box::new(comparisons::IfEq::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        154 => Ok(Box::new(comparisons::IfNe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        155 => Ok(Box::new(comparisons::IfLt::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        156 => Ok(Box::new(comparisons::IfGe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        157 => Ok(Box::new(comparisons::IfGt::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        158 => Ok(Box::new(comparisons::IfLe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        159 => Ok(Box::new(comparisons::IfICmpEq::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        160 => Ok(Box::new(comparisons::IfICmpNe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        161 => Ok(Box::new(comparisons::IfICmpLt::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        162 => Ok(Box::new(comparisons::IfICmpGe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        163 => Ok(Box::new(comparisons::IfICmpGt::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        164 => Ok(Box::new(comparisons::IfICmpLe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        165 => Ok(Box::new(comparisons::IfACmpEq::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        166 => Ok(Box::new(comparisons::IfACmpNe::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        167 => Ok(Box::new(control::Goto::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        168 => Ok(Box::new(control::Jsr::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        169 => Ok(Box::new(control::Ret::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        170 => Ok(Box::new(control::TableSwitch::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        171 => Ok(Box::new(control::LookupSwitch::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        172 => Ok(Box::new(control::IReturn::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        173 => Ok(Box::new(control::LReturn::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        174 => Ok(Box::new(control::FReturn::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        175 => Ok(Box::new(control::DReturn::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        176 => Ok(Box::new(control::AReturn::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        177 => Ok(Box::new(control::Return::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        178 => Ok(Box::new(references::GetStatic::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        179 => Ok(Box::new(references::PutStatic::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        180 => Ok(Box::new(references::GetField::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        181 => Ok(Box::new(references::PutField::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        182 => Ok(Box::new(references::InvokeVirtual::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        183 => Ok(Box::new(references::InvokeSpecial::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        184 => Ok(Box::new(references::InvokeStatic::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        185 => Ok(Box::new(references::InvokeInterface::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        186 => Ok(Box::new(references::InvokeDynamic::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        187 => Ok(Box::new(references::New::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        188 => Ok(Box::new(references::NewArray::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        189 => Ok(Box::new(references::ANewArray::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        190 => Ok(Box::new(references::ArrayLength::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        191 => Ok(Box::new(references::AThrow::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        192 => Ok(Box::new(references::CheckCast::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        193 => Ok(Box::new(references::InstanceOf::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        194 => Ok(Box::new(references::MonitorEnter::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        195 => Ok(Box::new(references::MonitorExit::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        196 => Ok(Box::new(extended::Wide::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        197 => Ok(Box::new(extended::MultiANewArray::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        198 => Ok(Box::new(extended::IfNull::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        199 => Ok(Box::new(extended::IfNonNull::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        200 => Ok(Box::new(extended::GotoW::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        201 => Ok(Box::new(extended::JsrW::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        202 => Ok(Box::new(reserved::Breakpoint::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        
        254 => Ok(Box::new(reserved::ImpDep1::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        255 => Ok(Box::new(reserved::ImpDep2::new(v, c, jvm, was_wide, true_pc)?) as Box<dyn Instruction>),
        _ => {
            panic!("Error: Opcode {} not supported yet", op);
        },
    }
}