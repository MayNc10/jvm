use std::{env, fs::File, fs, io::Read};
use rust_jvm::{jvm::{JVM, settings}, class::{Class, classfile::ClassFile, self}, argsparser};

pub fn load_class(f: &mut File, path: &str) -> ClassFile {
    unsafe {
        let metadata = fs::metadata(&path).unwrap();
        let size = if metadata.len() % 8 == 0 {metadata.len() / 8} else {metadata.len() / 8 + 1};
        let buffer: Vec<u64> = vec![0; size as usize];
        let mut buf_bytes = std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut u8, buffer.len() * std::mem::size_of::<u64>());
        f.read(&mut buf_bytes).unwrap();
        ClassFile::new(&buf_bytes).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result_args = match argsparser::parse_args(&args) {
        Ok(cli) => cli,
        Err(e) => {
            match e {
                argsparser::ParseError::MissingFileArgument => println!("Missing argument: filename"),
                argsparser::ParseError::FileDoesNotExist(s) => println!("Provided file {} does not exist", s),
                argsparser::ParseError::MissingClassPathArgument => println!("Missing argument: class path"),
                
            }
            println!("Aborting due to previous error");
            return;
        }
    };
    let main_class_file = load_class(&mut result_args.file, &result_args.fpath);
    if result_args.should_dump {
        println!("Loaded Class: {}", main_class_file);
    }
    if result_args.should_run {
        let jvm = JVM::new_with_main_class(main_class_file, result_args.flags, result_args.classpath.clone()).unwrap();
        jvm.excecute();
    }

}
