use std::{process::Command, time::Instant};

use paste::paste;

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

    let actual = Command::new("./target/release/cmd").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    
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
    let actual = Command::new("./target/release/cmd").arg("-r").arg(true_file).output().expect("Failed to run jvm");
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

    let actual = Command::new("./target/release/cmd").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    
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
    let actual = Command::new("./target/release/cmd").arg("-r").arg(true_file).output().expect("Failed to run jvm");
    let time_after_jvm = Instant::now(); 

    assert_eq!(expected, actual);
    println!("Us: {:#?}, them: {:#?}", time_after_jvm.duration_since(time_before_jvm), time_after_java.duration_since(time_before_java));
    assert!(time_after_jvm.duration_since(time_before_jvm) < time_after_java.duration_since(time_before_java));
}

macro_rules! add_test_and_time {
    ($name:ident) => {
        #[test]
        fn $name() {
            let mut name = String::from(stringify!($name));
            name.as_mut_str()[0..1].make_ascii_uppercase();

            while let Some(idx) = name.find("_") {
                name.as_mut_str()[idx + 1..idx + 2].make_ascii_uppercase();
                name.replace_range(idx..idx+1, "");
            }


            test_chapter(&module_path!()[module_path!().rfind(":").unwrap() + 1..], 
                name.as_str());
        }
        paste! {
            #[test]
            fn [<$name _timed>] () {
                let mut name = String::from(stringify!($name));
                name.as_mut_str()[0..1].make_ascii_uppercase();

                while let Some(idx) = name.find("_") {
                    name.as_mut_str()[idx + 1..idx + 2].make_ascii_uppercase();
                    name.replace_range(idx..idx+1, "");
                }

                test_chapter_timed(&module_path!()[module_path!().rfind(":").unwrap() + 1..], 
                    name.as_str());
            }
        }     
    };
}

mod book {
    use super::*;
    mod ch01 {
        use super::*;
        add_test_and_time!(hello);
        add_test_and_time!(hello2);
        add_test_and_time!(hello3);
        add_test_and_time!(goodbye);
    }
    mod ch02 {
        use super::*;
        add_test_and_time!(declare_assign); 
        add_test_and_time!(floating_point);
        add_test_and_time!(hello);
        add_test_and_time!(memory_diagram);
        add_test_and_time!(printing_vars);
        add_test_and_time!(string_concat);
    }
    mod ch03 {
        use super::*;
        add_test_and_time!(convert);
        add_test_and_time!(echo);
        add_test_and_time!(formatting);
        add_test_and_time!(guess_starter);
        add_test_and_time!(literals);
        add_test_and_time!(scanner_bug);
    }
}



mod speed {
    use super::*;

    #[test]
    fn primes() {
        test_file_timed("speed", "Primes");
    }
}
