fn main() {
    struct Person {
        name: String,
        age: u32,
        address: String,
        online: bool,
    }

    let p: Person = Person {
        name: String::from("Robin"),
        age: 25,
        address: String::from("Codeway 99"),
        online: true,
    };

    println!("Person first name is: {}, age = {}, address = {}, online = {}", 
        p.name,
        p.age,
        p.address,
        p.online
    );
}
