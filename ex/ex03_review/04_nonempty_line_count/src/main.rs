use ex03_nonempty_line_count::count_nonempty_lines;

fn main() -> Result<(), std::io::Error> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/sample.txt");
    let count = count_nonempty_lines(path)?;
    println!("{count}");
    Ok(())
}

