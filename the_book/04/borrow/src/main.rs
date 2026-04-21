fn main() {
    let mut i = 0;

    while i < 10 {
        let mut j = 0;
        while j < 3 {
            println!("{i}, {j}");
            j += 1;
        }
        i += 1;
    }
}
