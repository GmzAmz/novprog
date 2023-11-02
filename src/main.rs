use std::io;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut input: String = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input)?;
    println!("input: {}", input);
    Ok(())
}
