use kial_compiler::env::Env;

fn main() {
    let mut env = Env::default();

    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        if let Ok(_) = stdin.read_line(&mut input) {
            let val = kial_compiler::parse(input.as_str(), &mut env);

            match val {
                Ok(val) => {
                    println!("> {val}");
                }
                Err(e) => {
                    println!("> {e}")
                }
            }
        }
    }
}
