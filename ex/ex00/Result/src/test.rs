fn main() -> Result<(), String> {
    if 1 < 3 {
        return Result::Err("Hello".to_string());
    }
    Ok(())
}
