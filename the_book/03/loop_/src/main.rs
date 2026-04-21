fn main() {
    for number in (1..4) {
        println!("{number}");
    }
    for number in (1..4).rev() {
        println!("{number}");
    }
    let mut count = 0;
    let x = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 3;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}");
    println!("{x}");
}
