
fn sum(xs: [i32]) -> i32 {
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn main() {

    println!("{}", sum([1, 2, 3]));

    let x = 10;
    let y = &x;

    if y == 10 {
        println!("hi");
    }

}
