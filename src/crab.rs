use std::fs;
use std::io;
use std::iter::zip;
use std::thread;
use std::time::Duration;

use crate::utils;

pub fn open(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn write(path: &String, contents: &String) -> Result<(), io::Error> {
    fs::write(path, contents)
}

pub fn read(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => println!("\n{}", val),
        Err(_) => {}
    }
}

pub fn lines(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            println!("");
            for (i, v) in val.lines().enumerate() {
                println!("[{}] {}", i + 1, v);
            }
            println!("");
        }
        Err(_) => {}
    }
}

pub fn find(arg: &Vec<String>) {
    let search = &arg[3].to_lowercase();

    match open(&arg[2]) {
        Ok(val) => {
            println!("");
            for (i, v) in val.lines().enumerate() {
                if v.to_lowercase().contains(search) {
                    println!("[{}] {}", i, v);
                }
            }
            println!("");
        }
        Err(_) => {}
    }
}

pub fn count(arg: &Vec<String>) {
    let search = &arg[3].to_lowercase();

    match open(&arg[2]) {
        Ok(val) => {
            println!("{}", val.to_lowercase().matches(search.as_str()).count())
        }
        Err(_) => {}
    }
}

pub fn len(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            println!("{}", utils::len(&val))
        }
        Err(_) => {}
    }
}

pub fn lenline(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            println!("{}", utils::line_count(&val))
        }
        Err(_) => {}
    }
}

pub fn slice(arg: &Vec<String>) {
    let rng: Vec<&str> = arg[3].split('-').collect();

    let s1: usize = rng[0].trim().parse().expect("Not a number");
    let s2: usize = rng[1].trim().parse().expect("Not a number");

    let mut result = String::new();

    match open(&arg[2]) {
        Ok(val) => {
            for (i, v) in val.chars().enumerate() {
                if (s1..s2).contains(&i) {
                    result.push(v);
                } else if i == s2 {
                    break;
                }
            }

            println!("{}", result);
        }
        Err(_) => {}
    }
}

pub fn replace(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            write(&arg[2], &val.replace(&arg[3], &arg[4])).expect("Error");
            println!("{}", open(&arg[2]).unwrap())
        }
        Err(_) => {}
    }
}

pub fn reverse(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            for i in val.lines() {
                println!("{}", utils::reverse(&i.to_string()));
            }
        }
        Err(_) => {}
    }
}

pub fn mask(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(val) => {
            let mut mask = String::new();

            for _ in 0..utils::len(&arg[3]) {
                mask.push('*');
            }

            write(&arg[2], &val.replace(&arg[3], &mask)).expect("Error");
            println!("{}", open(&arg[2]).unwrap())
        }
        Err(_) => {}
    }
}

pub fn rewrite(arg: &Vec<String>) {
    match open(&arg[2]) {
        Ok(_) => {
            write(&arg[2], &arg[3]).unwrap();
        }
        Err(_) => {}
    }
}

pub fn watch(arg: &Vec<String>) {
    let x = open(&arg[2]).expect("");

    let handle = thread::spawn({
        let thrg = arg.clone();
        let mut x = x.clone();

        move || {
            loop {
                if open(&thrg[2]).expect("") != x {
                    x = open(&thrg[2]).expect("");
                    println!("{}", x);
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    handle.join().unwrap();
}

pub fn diff(arg: &Vec<String>) {
    check(&open(&arg[2]).expect(""), &open(&arg[3]).expect(""));

    fn check(janedoy: &String, johndoy: &String) {
        for (i1, i2) in zip(janedoy.lines(), johndoy.lines()) {
            if i1 != i2 {
                println!("{} - {}", i1, i2);
            }
        }
    }
}

pub fn compare(arg: &Vec<String>) {
    check(&open(&arg[2]).expect(""), &open(&arg[3]).expect(""));

    fn check(janedoy: &String, johndoy: &String) {
        for i in janedoy.lines() {
            if johndoy.contains(i) == false {
                println!("{}", i)
            }
        }
    }
}

pub fn help() {
    println!(
        r#"
    Usage: crab <COMMAND> [ARGS]

    Commands:
      read     <FILE>                Print entire file contents
      line     <FILE>                Print file lines with numeric prefixes
      slice    <FILE> <START-END>    Extract text segment by character range

      find     <FILE> <PATTERN>      Search lines for matching pattern (case-insensitive)
      count    <FILE> <PATTERN>      Count total occurrences of a specific pattern
      reverse  <FILE>                Reverse character order of every line in file

      replace  <FILE> <OLD> <NEW>    Replace matching patterns globally within file
      mask     <FILE> <PATTERN>      Censor target pattern with masks matching length
      rewrite  <FILE> <TEXT>         Overwrite entire file with a single text line

      len      <FILE>                Count total Unicode characters (UTF-8 safe)
      lenline  <FILE>                Count total lines based on trailing line feeds

      watch    <FILE>                Real-time multi-threaded log file monitoring
      diff     <FILE1> <FILE2>       Line-by-line structural divergence verification
      compare  <FILE1> <FILE2>       Isolated unique content matching matrix

    Examples:
      crab read data.log
      crab slice context.db 10-25
      crab find evidence.txt "Тень"
      crab mask server.conf secret_token
      crab watch production.stream
      crab compare legacy.csv snapshot.csv
    "#
    );
}
