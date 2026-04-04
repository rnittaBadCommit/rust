enum Value {
    Int(i32),
    Float(f32),
}

fn print_value(v: Value) {
    match v {
        Value::Int(i) => println!("int: {i}"),
        Value::Float(f) => println!("float: {f}"),
    }
}

fn main() {
    let v = Value::Int(42);
    print_value(v);

    let v = Value::Float(3.14);
    print_value(v);

    let v = Value::Float(42.0);
    print_value(v);

    let values = vec![Value::Int(10), Value::Float(2.6), Value::Int(-7)];
    for v in values {
        print_value(v);
    }
}
