use rust_jvm::load_class;
use send_wrapper::SendWrapper;
use once_cell::sync::Lazy;

use std::mem::size_of;
use std::{env, fs::File, fs, io::Read};
use rust_jvm::{jvm::JVM, class::classfile::ClassFile, argsparser};

#[cfg(not(target_family = "wasm"))]
use {
    inkwell::context::Context,
    inkwell::targets::{InitializationConfig, Target},
    inkwell::execution_engine::ExecutionEngine,
};

#[cfg(not(target_family = "wasm"))]
static CONTEXT: Lazy<SendWrapper<Context>> = Lazy::new(|| SendWrapper::new(Context::create()));

fn main() {
    #[cfg(not(target_family = "wasm"))]
    Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");
    #[cfg(not(target_family = "wasm"))]
    ExecutionEngine::link_in_mc_jit();

    let args: Vec<String> = env::args().collect();
    let mut result_args = match argsparser::parse_args(&args) {
        Ok(cli) => cli,
        Err(e) => {
            match e {
                argsparser::ParseError::MissingFileArgument => println!("Missing argument: filename"),
                argsparser::ParseError::FileDoesNotExist(s) => println!("Provided file {s} does not exist"),
                argsparser::ParseError::MissingClassPathArgument => println!("Missing argument: class path"),
                
            }
            println!("Aborting due to previous error");
            return;
        }
    };
    let (main_class_file, code_bytes) = load_class(&mut result_args.file, &result_args.fpath);
    #[cfg(not(target_family = "wasm"))]
    let mut jvm = JVM::new_with_main_class(main_class_file, code_bytes, result_args.flags, result_args.classpath.clone(), &CONTEXT).unwrap();
    #[cfg(target_family = "wasm")]
    let mut jvm = JVM::new_with_main_class(main_class_file, code_bytes, result_args.flags, result_args.classpath.clone()).unwrap();
    if result_args.should_dump {
        println!("Loaded Class: {}", jvm.resolve_class_reference(jvm.m_main_class_name.clone().as_str()).unwrap().get_class_file());
    }
    if result_args.should_run {
        jvm.excecute();
    }

}
