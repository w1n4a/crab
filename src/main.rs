use std::env::args;
mod crab;
mod utils;

fn main() {
    let arg: Vec<String> = args().collect();

    if arg.len() == 1 {
        crab::help();
    } else {
        match arg[1].as_str() {
            "read" => crab::read(&arg),
            "line" => crab::lines(&arg),
            "find" => crab::find(&arg),
            "count" => crab::count(&arg),
            "len" => crab::len(&arg),
            "lenline" => crab::lenline(&arg),
            "slice" => crab::slice(&arg),
            "replace" => crab::replace(&arg),
            "reverse" => crab::reverse(&arg),
            "mask" => crab::mask(&arg),
            "rewrite" => crab::rewrite(&arg),
            "watch" => crab::watch(&arg),
            "diff" => crab::diff(&arg),
            "compare" => crab::compare(&arg),
            "help" => crab::help(),
            _ => crab::help(),
        }
    }
}
