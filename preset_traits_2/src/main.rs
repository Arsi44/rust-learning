use std::cmp::Ordering;
use std::ops::Add;
use std::ops::AddAssign;


// PartialEq Example
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

// Add Example (very similar for Sub and Rem)
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// AddAssign Example
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point2 {
    x: i32,
    y: i32,
}

impl AddAssign for Point2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


fn main() {

// PartialEq Example
    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    let b3 = Book { isbn: 10, format: BookFormat::Paperback };

    assert!(b1 == b2);
    assert!(b1 != b3);

    // PartialOrd example
    let x: u32 = 0;
    let y: u32 = 1;

    assert_eq!(x < y, true);
    assert_eq!(x.lt(&y), true);

    // Ordering Example
    assert_eq!(1.cmp(&2), Ordering::Less);
    assert_eq!(1.cmp(&1), Ordering::Equal);
    assert_eq!(2.cmp(&1), Ordering::Greater);

    // Add Example (very similar for Sub and Rem)
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 });

    let mut point = Point2 { x: 1, y: 0 };
    point += Point2 { x: 2, y: 3 };
    println!("{:?}", point);  // Point2 { x: 3, y: 3 }
}