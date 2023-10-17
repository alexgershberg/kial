use kial_compiler::env::Env;

fn main() {
    let mut env = Env::default();

    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        if let Ok(_) = stdin.read_line(&mut input) {
            match kial_compiler::parse(input.trim()) {
                Ok(parse) => {
                    // println!("> {parse:#?}");
                    let res = parse.eval(&mut env);
                    println!("> {res:?}");
                }
                Err(e) => {
                    println!("> {e}")
                }
            }
        }
    }
}
