mod house;

use house::{bedroom1, bedroom2};


fn main() {
    println!("Hello, world!");
    println!("{}", bedroom1::light_is_on());
    println!("{}", bedroom1::is_neighbour_light_on());
    println!("{}", bedroom2::light_is_on());
    println!("{}", house::HOUSE_NUMBER);
}

#[cfg(test)]
mod tests {
    use super::*;  // Import all known names for outer area

    #[test]
    fn check_equality() {
        assert_eq!(bedroom1::light_is_on(), false);
    }

    #[test]
    fn check_bound() {
        assert_eq!(bedroom1::is_neighbour_light_on(), true);
    }
}
