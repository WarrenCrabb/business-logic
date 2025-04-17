use business_logic::repl;
use std::io::{self, stdin, stdout};
fn main() -> io::Result<()> {
    println!(
        "Hello trainer! This is the PokeScript language!",
        // username
    );
    println!("Feel free to type in commands");

    // Start the REPL using standard input and output.
    repl::start(stdin().lock(), stdout())?;
    Ok(())
}
