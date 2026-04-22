use crate::garden::vegetables::Asparagas;

pub mod garden;

fn main() {
    let plant = Asparagas {};
    println!("Im growing {:?}!", plant);
}