struct Person {
    name: String,
    age: u32,
    address: String,
    car: Car,
    online: bool,
}

struct Car {
    model: String,
    year: String,
    brand: String,
}

// Unit-Like Structs Without Any Fields
// can be useful when you need to implement a trait on some type 
// but don’t have any data that you want to store in the type itself.
struct Placeholder;

#[derive(Debug)]
struct Coordinates(f32, f32);

fn main() {
    let mut p: Person = Person {
        name: String::from("Robin"), // can't put mut on specific fields
        age: 25,
        address: String::from("Codeway 99"),
        car: Car {
            model: String::from("Tesla Plaid"),
            year: String::from("2022"),
            brand: String::from("Tesla"),
        },
        online: true,
    };

    println!("Person first name is: {}, age = {}, address = {}, online = {}", 
        p.name,
        p.age,
        p.address,
        p.online
    );

    println!("Person car brand: {}, model: {}, year: {}", 
        p.car.brand,
        p.car.model,
        p.car.year
    );

    p.name = String::from("Lobin");

    println!("Person first name is now: {}", p.name);

    // create person and print age
    let p2 = create_person(
        String::from("Lubin"),
        33,
        String::from("Codeway 124"),
        false,
    );

    println!("Person 2's age is: {}", p2.age);

    let p3 = Person {
        age: 100,
        ..p2        // <- .. specifies that the remaining fields not explicitly set 
                    // should have the same value as the fields in the given instance
    };

    println!("Person 3 name = {}, age = {}", p3.name, p3.age);

    let coordinates = Coordinates(54.383727222, 23.42832632);

    println!("{:?}", coordinates);

    let placeholder = Placeholder;
}

// Create person and return person struct populated with data
fn create_person(name: String, age: u32, address: String, online: bool) -> Person {
    Person {
        name,
        age,
        address,
        car: Car {
            model: String::from("Tesla Plaid"),
            year: String::from("2022"),
            brand: String::from("Tesla"),
        },
        online,
    }
}
