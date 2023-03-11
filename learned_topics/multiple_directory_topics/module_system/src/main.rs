mod garden;

use crate::garden::vegetables::Carrot;

fn main() {
    let plant = Carrot {};
    println!("I'm growing {:?}!", plant);
}
