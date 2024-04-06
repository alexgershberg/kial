use old_kial_compiler::env::Env;
use std::io::Write;

fn main() {
    let mut env = Env::default();

    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        if let Ok(_) = stdin.read_line(&mut input) {
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match old_kial_compiler::parse(input) {
                Ok(parse) => match parse.eval(&mut env) {
                    Ok(val) => println!("{val}"),
                    Err(e) => {
                        println!("1: {e}")
                    }
                },
                Err(e) => {
                    println!("2: {e}")
                }
            }
        }
    }
}
