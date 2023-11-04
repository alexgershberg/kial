# Kial

### Kial - "Why" in Esperanto (name subject to change). 
***"Why"***, as in, *"***Why*** would you want to ***DO THIS?***"*

I got inspired by this series of blog posts, and have decided following them to implement my own simple programming language.
https://lunacookies.github.io/lang/

#### Build using `cargo build`
#### Run REPL using `cargo run`

The language has Rust-like syntax for simplicity

this is *sooooo untested* 
```rust
func main() {
  let a = "Hello";
  let b = ", world!";
  a + b
}

main()
```

This should in theory print "Hello, world!" in the REPL. However, the REPL doesn't currently support multiline input, so it would have to be entered like:
```rust
func main() { let a = "Hello"; let b = ", world!"; a + b }
main()
```
OR
```rust
let a = "Hello";
let b = ", world!"
a + b
```

(Maybe the language should be called, *"***Why*** would you want to ***USE THIS?***"*)
