use std::{env, fs::{self, File}, io::Read, process::Command, fmt::format, time::Instant};

use crate::class::classfile::ClassFile;

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

pub fn test_chapter_time(chap: &str, file: &str) {
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
    assert!(time_after_jvm.duration_since(time_before_jvm) < time_after_java.duration_since(time_before_java));
}


#[test]
fn ch01_hello() {
    test_chapter("ch01", "Hello")
}

#[test]
fn ch01_hello_time() {
    test_chapter_time("ch01", "Hello")
}

#[test]
fn ch01_hello2() {
    test_chapter("ch01", "Hello2")
}

#[test]
fn ch01_hello2_time() {
    test_chapter_time("ch01", "Hello2")
}

#[test]
fn ch01_hello3() {
    test_chapter("ch01", "Hello3")
}

#[test]
fn ch01_hello3_time() {
    test_chapter_time("ch01", "Hello3")
}

#[test]
fn ch01_goodbye() {
    test_chapter("ch01", "Goodbye")
}

#[test]
fn ch01_goodbye_time() {
    test_chapter_time("ch01", "Goodbye")
}