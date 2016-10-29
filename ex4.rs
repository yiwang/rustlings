// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    match "3x".parse::<i32>() {
        Ok(x) => Ok(x * 4),
        Err(err) => Err(err),
    }
}

fn main() {
    match something() {
        Ok(..) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
