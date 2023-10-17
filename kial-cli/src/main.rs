use kial_compiler::env::Env;

fn main() {
    let mut env = Env::default();
    loop {
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
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
