
    pub fn light_is_on() -> bool {
        false
    }

    pub fn is_neighbour_light_on() -> bool {
        use super::bedroom2;
        bedroom2::light_is_on()
    }
