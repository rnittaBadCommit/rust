fn print_len(s: &String) {
    println!("{}", s.len());
}

fn main() {
    let mut s = String::from("Hello");
    print_len(&s);
    println!("{s}");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;
    r3.push_str(" world");
    println!("{r1} {r3}");
}
