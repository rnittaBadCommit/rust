fn main() {
    let mut v = vec![10, 20, 30];

    let mut v2 = &mut v;
    let mut v3 = v2;

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
