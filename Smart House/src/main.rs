fn main() {
    let mut rosette = Rosette {
        mode: String::from("off"),
        power: 101,
    };

    rosette.show_power();
    println!("Mode == {}", &rosette.mode);
    rosette.change_mode();
    println!("Mode == {}", &rosette.mode);


    let mut thermometer = Thermometer {
        degrees: 36.6,
    };
    thermometer.show_degrees();

}

struct Rosette {
    mode: String,
    power: i16,
}

impl Rosette {
    fn description() {
        println!("Rosette Description...")
    }

    fn change_mode(&mut self) {
        if self.mode == "off".to_string() {
            self.mode = "on".to_string()
        } else {
            self.mode = "off".to_string();
        }
    }

    fn show_power(&self) {
        println!("Power == {} Bt", self.power)
    }
}

struct Thermometer {
    degrees: f32,
}

impl Thermometer {
    fn show_degrees(&self) {
        todo!()
    }
}