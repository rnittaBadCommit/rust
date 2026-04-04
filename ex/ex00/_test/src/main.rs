fn main() {
    let mut x = 42;
    {
        let y = &mut x;

        let mut z = 1;
        z = y;
        println!("{z}");
    }
    x = 10;
    println!("{x}");
}
