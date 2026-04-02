fn sum(xs: &[i32]) -> i32 {
    let mut total = 0;
    for x in xs {
        total += *x;
    }
    total
}

fn main() {
    let a = [1, 2, 3, 4];

    println!("{}", sum(&a));
    println!("{}", sum(&a[1..3]));
}
