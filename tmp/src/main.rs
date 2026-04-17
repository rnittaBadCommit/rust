fn f() {
}

fn g() -> () {
    5;
    ()
}


fn main() {
    let x = f();
    let y = g();
    println!("{x:?}");
    if x == y {
        println!("x == y");
    }
}
