use std::fs::File;

use crate::jvm;

#[derive(PartialEq)]
pub enum FileType {
    Class,
    Jar,
}
pub struct Cli {
    pub ftype: FileType,
    pub file: File,
    pub fpath: String,
    // args
    pub should_run: bool,
    pub verbose: bool,
    pub should_dump: bool,
    pub classpath: Option<String>,

    pub flags: u8,
}

pub enum ParseError {
    FileDoesNotExist(String),
    MissingFileArgument,
    MissingClassPathArgument,
}

pub fn parse_args(args: &[String]) -> Result<Cli, ParseError> {
    let mut should_run = false;
    let mut should_dump = false;
    let mut verbose = false;
    let mut flags = 0;
    let mut fname = String::from("");
    let mut ftype = FileType::Class;
    let mut classpath = None;
    if args.contains(&String::from("-d")) {
        should_dump = true;
    }
    if args.contains(&String::from("--dump")) {
        should_dump = true;
    }
    if args.contains(&String::from("-r")) {
        should_run = true;
    }
    if args.contains(&String::from("--run")) {
        should_run = true;
    }
    if args.contains(&String::from("-j")) {
        ftype = FileType::Jar;
    }
    if args.contains(&String::from("--jar")) {
        ftype = FileType::Jar;
    }
    if args.contains(&String::from("-ac")) {
        flags |= jvm::settings::SHOULD_CONTROL_ACCESS;
    }
    if args.contains(&String::from("--access-control")) {
        flags |= jvm::settings::SHOULD_CONTROL_ACCESS;
    }
    if args.contains(&String::from("-v")) {
        flags |= jvm::settings::SHOULD_VERIFY;
    }
    if args.contains(&String::from("--verify")) {
        flags |= jvm::settings::SHOULD_VERIFY;
    }
    if args.contains(&String::from("-db")) {
        flags |= jvm::settings::SHOULD_BACKTRACE;
    }
    if args.contains(&String::from("--dump-backtrace")) {
        flags |= jvm::settings::SHOULD_BACKTRACE;
    }
    if args.contains(&String::from("-v")) {
        verbose = true;
    }
    if args.contains(&String::from("--verbose")) {
        verbose = true;
    }
    if let Some(sindex) = args.iter().position(|s| s == "-cp") {
        if args.len() == sindex + 1 {
            return Err(ParseError::MissingClassPathArgument);
        }
        classpath = Some(args[sindex + 1].clone());
    }
    if let Some(sindex) = args.iter().position(|s| s == "--classpath") {
        if args.len() == sindex + 1 {
            return Err(ParseError::MissingClassPathArgument);
        }
        classpath = Some(args[sindex + 1].clone());
    }
    let mut missing_file = args.is_empty();
    if !missing_file {
        fname = args.last().unwrap().clone();
        if classpath.is_some() {
            fname = classpath.as_ref().unwrap().clone() + &fname;
        }
        if ftype == FileType::Class {
            if fname.len() <= 6 {
                missing_file = true;
            }
            else {
                missing_file = &fname[(fname.len() - 6)..] != ".class"
            }
        }
        else if fname.len() <= 4 {
            missing_file = true;
        }
        else {
            missing_file = &fname[(fname.len() - 6)..] != ".jar"
        }
        
    }
    if missing_file {
        return Err(ParseError::MissingFileArgument);
    }
    let file = match File::open(&fname) {
        Ok(f) => f,
        Err(_) => return Err(ParseError::FileDoesNotExist(fname)),
    };
    Ok(Cli {
        ftype,
        file,
        fpath: fname, // This should change to look in a specific class path
        should_run,
        should_dump,
        verbose,
        classpath,
        flags,
    })
}