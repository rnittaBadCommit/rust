enum Value {
    Int(i32),
    Float(f32),
}

fn print_value(v: &Value) {
    match v {
        Value::Int(i) => println!("int: {i}"),
        Value::Float(f) => println!("float: {f}"),
    }
}

fn describe(n: i32) {
    match n {
        ..0 => println!("minus"),
        0 => println!("zero"),
        1 | 2 => println!("small"),
        3..=9 => println!("medium"),
        _ => println!("large"),
    }
}

fn main() {
    for i in -10..10 {
        println!("{i}");
        describe(i);
    }

    let mut v = Value::Int(42);
    print_value(&v);

    v = Value::Float(3.14);
    print_value(&v);

    v = Value::Float(42.0);
    print_value(&v);

    let values = vec![Value::Int(10), Value::Float(2.6), Value::Int(-7)];
    for v in values {
        print_value(&v);
    }
}
