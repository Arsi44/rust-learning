mod house;

use house::{bedroom1, bedroom2};


fn main() {
    println!("Hello, world!");
    println!("{}", bedroom1::light_is_on());
    println!("{}", bedroom1::is_neighbour_light_on());
    println!("{}", bedroom2::light_is_on());
    println!("{}", house::HOUSE_NUMBER);
}
