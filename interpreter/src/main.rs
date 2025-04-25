// use business_logic::repl;
use business_logic::script;
use std::fs;
use std::io::{self, Result, stdout};
// use std::io::{self, Result, stdin, stdout};

fn read_whole_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
}

fn main() -> io::Result<()> {
    // println!("Hello! Start writing some Business logic!");
    // repl::start(stdin().lock(), stdout())?;

    // let script = read_whole_file("./test.biz");
    let script = read_whole_file("./closure.biz");
    // let script = read_whole_file("./recur.biz");

    if script.is_ok() {
        let script = script.unwrap();
        script::run_script(&script, stdout())?
    }

    Ok(())
}
