use std::fs::File;
use std::io::ErrorKind;
use std::convert::From;

// Debug
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Error Example
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SuperError {
    source: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError { source: SuperErrorSideKick })
}

// From-Into example
#[derive(Debug)]
struct GFG {
    year: i32,
}
 
impl From<i32> for GFG {
    fn from(item: i32) -> Self {
        GFG {year: item }
    }
}



fn main() {
    let origin = Point { x: 0, y: 0 };
    assert_eq!(format!("The origin is: {origin:?}"), "The origin is: Point { x: 0, y: 0 }");
    // println!("{origin}") // Error - `Point` cannot be formatted with the default formatter
    println!("{origin:?}");


    // ToString
    let i = 5;
    let five = String::from("5");
    assert_eq!(five, i.to_string());
    println!("{} {}", i.to_string(), five);


    // Error
    let err = "NaN".parse::<u32>().unwrap_err();
    assert_eq!(err.to_string(), "invalid digit found in string");
    println!("{}", err);

    match get_super_error() {
        Err(e) => {
            println!("Error: {e}");
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    };

    let mut x = get_super_error();
    println!("{x:?}");  // Err(SuperError { source: SuperErrorSideKick })

    // Error Example with panic!
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // From-Into example
    let num_1 = GFG::from(2023);
    println!("learning Rust from {:?}", num_1);
    let int_var = 2023;
    let num_2: GFG = int_var.into();
    println!("Learning Rust from {:?}", num_2);

}
