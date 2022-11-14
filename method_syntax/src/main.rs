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
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 50,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect.area()
    );
}
