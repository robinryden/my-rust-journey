#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods are defined within the context of a struct (or an enum or a trait object
// Everything within this impl block will be associated with the Rectangle type
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    let other_rect = Rectangle {
        width: 49,
        height: 35,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect.area()
    );

    if rect.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rect.width
        );
    }

    println!("Can rect hold other_rect? {}", rect.can_hold(&other_rect));

    let sq = Rectangle::square(10);
    dbg!("sq = {}", sq);
}
