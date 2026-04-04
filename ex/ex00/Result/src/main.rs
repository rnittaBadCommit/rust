use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), std::io::Error> {
    let (text, message) = match read_file("hello.txt") {
        Ok(text) => (text, "success!!"),
        Err(err) => return Err(err),
    };

    println!("{message}");
    println!("{text}");
    Ok(())
}
