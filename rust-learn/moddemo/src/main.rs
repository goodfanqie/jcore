





use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I am growing {:?}", plant);
    println!("Hello, world!");
}
