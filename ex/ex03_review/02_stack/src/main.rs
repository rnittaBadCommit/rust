use ex03_stack::Stack;

fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    while let Some(value) = stack.pop() {
        println!("{value}");
    }
}

