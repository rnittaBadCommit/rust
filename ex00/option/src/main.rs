fn find_even(xs: &[i32]) -> Option<i32> {
    for x in xs {
        if x % 2 == 0 {
            return Some(*x);
        }
    }
    None
}

fn main() {
    let a = [1, 3, 5, 3];
    match find_even(&a) {
        Some(v) => println!("found: {v}"),
        None => println!("not found"),
    }
}
