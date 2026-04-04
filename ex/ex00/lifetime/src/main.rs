use std::collections::HashMap;

fn greet<'a>(id: i32, name: &str, name2: &'a str) -> HashMap<i32, &'a str> {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(id, name2);
    map.insert(id, name2);
    map
}

fn f<'a>(name2: &'a str) -> HashMap<i32, &'a str> {
    let name = String::from("rnitta");

    let res = greet(42, &name, name2);
    println!("{:?}", res);
    res
}

fn main() {
    let name2 = String::from("zzz");
    let res = f(&name2);
    println!("{:?}", res);
}
