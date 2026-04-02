struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn norml(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn into_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() {
    let mut p = Point { x: 3, y: -4 };

    println!("{}", p.norml());

    p.translate(10, 20);

    println!("{}", p.norml());

    Point::translate(&mut p, 1, 1);

    println!("{}", Point::norml(&mut p));

    let t = p.into_tuple();
    println!("{t:?}");
}
