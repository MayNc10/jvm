use std::{process::Command, time::Instant};


pub fn test_chapter(chap: &str, file: &str) {
    let mut s = String::from(".");
    if let Ok(news) = std::env::var("JVM_FOLDER_PATH") {
        s = news;
    }

    let path = format!("{}/ThinkJavaCode2/{}", s, chap);
    let path = path.as_str();

    let true_file = format!("{}/{}.class", path, file);
    let true_file = true_file.as_str();

    let expected = Command::new("java").arg("-cp").arg(path).arg(file).output().expect("Failed to run java file from cli");

    let actual = Command::new("./target/release/rust-jvm").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    
    assert_eq!(expected, actual);
}

pub fn test_chapter_timed(chap: &str, file: &str) {
    let mut s = String::from(".");
    if let Ok(news) = std::env::var("JVM_FOLDER_PATH") {
        s = news;
    }

    let path = format!("{}/ThinkJavaCode2/{}", s, chap);
    let path = path.as_str();

    let true_file = format!("{}/{}.class", path, file);
    let true_file = true_file.as_str();

    let time_before_java = Instant::now();
    let expected = Command::new("java").arg("-cp").arg(path).arg(file).output().expect("Failed to run java file from cli");
    let time_after_java = Instant::now(); 

    let time_before_jvm = Instant::now();
    let actual = Command::new("./target/release/rust-jvm").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    let time_after_jvm = Instant::now(); 

    assert_eq!(expected, actual);
    println!("Us: {:#?}, them: {:#?}", time_after_jvm.duration_since(time_before_jvm), time_after_java.duration_since(time_before_java));
    assert!(time_after_jvm.duration_since(time_before_jvm) < time_after_java.duration_since(time_before_java));
}

pub fn test_file(path: &str, file: &str) {
    let mut s = String::from(".");
    if let Ok(news) = std::env::var("JVM_FOLDER_PATH") {
        s = news;
    }

    let path = format!("{}/{}", s, path);
    let path = path.as_str();

    let true_file = format!("{}/{}.class", path, file);
    let true_file = true_file.as_str();

    let expected = Command::new("java").arg("-cp").arg(path).arg(file).output().expect("Failed to run java file from cli");

    let actual = Command::new("./target/release/rust-jvm").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    
    assert_eq!(expected, actual);
}

pub fn test_file_timed(path: &str, file: &str) {
    let mut s = String::from(".");
    if let Ok(news) = std::env::var("JVM_FOLDER_PATH") {
        s = news;
    }

    let path = format!("{}/{}", s, path);
    let path = path.as_str();

    let true_file = format!("{}/{}.class", path, file);
    let true_file = true_file.as_str();

    let time_before_java = Instant::now();
    let expected = Command::new("java").arg("-cp").arg(path).arg(file).output().expect("Failed to run java file from cli");
    let time_after_java = Instant::now(); 

    let time_before_jvm = Instant::now();
    let actual = Command::new("./target/release/rust-jvm").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    let time_after_jvm = Instant::now(); 

    assert_eq!(expected, actual);
    println!("Us: {:#?}, them: {:#?}", time_after_jvm.duration_since(time_before_jvm), time_after_java.duration_since(time_before_java));
    assert!(time_after_jvm.duration_since(time_before_jvm) < time_after_java.duration_since(time_before_java));
}

mod ch01 {
    use super::*;
    #[test]
    fn hello() {
        test_chapter("ch01", "Hello")
    }
    #[test]
    fn hello_timed() {
        test_chapter_timed("ch01", "Hello")
    } 
    #[test]
    fn hello2() {
        test_chapter("ch01", "Hello2")
    } 
    #[test]
    fn hello2_timed() {
        test_chapter_timed("ch01", "Hello2")
    }
    #[test]
    fn hello3() {
        test_chapter("ch01", "Hello3")
    }
    #[test]
    fn hello3_timed() {
        test_chapter_timed("ch01", "Hello3")
    } 
    #[test]
    fn goodbye() {
        test_chapter("ch01", "Goodbye")
    }
    #[test]
    fn goodbye_timed() {
        test_chapter_timed("ch01", "Goodbye")
    }
}
mod ch02 {
    use super::*;
    #[test]
    fn declare_assign() {
        test_chapter("ch02", "DeclareAssign")
    }
    #[test]
    fn declare_assign_timed() {
        test_chapter_timed("ch02", "DeclareAssign")
    } 
    #[test]
    fn floating_point() {
        test_chapter("ch02", "FloatingPoint")
    }
    #[test]
    fn floating_point_timed() {
        test_chapter_timed("ch02", "FloatingPoint")
    } 
    #[test]
    fn hello() {
        test_chapter("ch02", "Hello")
    }
    #[test]
    fn hello_timed() {
        test_chapter_timed("ch02", "Hello")
    } 
    #[test]
    fn memory_diagram() {
        test_chapter("ch02", "MemoryDiagram")
    }
    #[test]
    fn memory_diagram_timed() {
        test_chapter_timed("ch02", "MemoryDiagram")
    } 
    #[test]
    fn printing_vars() {
        test_chapter("ch02", "PrintingVars")
    }
    #[test]
    fn printing_vars_timed() {
        test_chapter_timed("ch02", "PrintingVars")
    } 
    #[test]
    fn string_concat() {
        test_chapter("ch02", "StringConcat")
    }
    #[test]
    fn string_concat_timed() {
        test_chapter_timed("ch02", "StringConcat")
    } 
}

mod speed {
    use super::*;

    #[test]
    fn primes() {
        test_file_timed("speed", "Primes");
    }
}
