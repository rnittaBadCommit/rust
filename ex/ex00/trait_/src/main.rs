use std::ops::Add;

fn max_of_two<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn add_2<T, U>(a: T, b: U) -> <T as Add<U>>::Output
where
    T: Add<U>,
{
    a + b
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    println!("{}", max_of_two(1, 3));
    println!("{}", add(1, 3));
    println!("{}", add_2(2.0, 3.14));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    println!("{:?}", p1 + p2);
}
