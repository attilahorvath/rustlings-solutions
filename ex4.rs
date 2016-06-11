// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    let x: Result<i32, std::num::ParseIntError> = "3".parse();
    match x {
        Ok(x) => Ok(x * 4),
        e => e,
    }
}

fn main() {
    match something() {
        Ok(..) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
