fn main() {
    let mut v = vec![10, 20, 30];

    for x in &mut v {
        *x *= 2;
    }

    println!("{v:?}");

    let mut sum = 0;
    for x in v {
        sum += x;
    }
    println!("{sum}");
}
