fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        
        if count == 25 {
            break count * 4;
        }
    };

    println!("Result of count is: {result}");

    let mut number = 10;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Finished!");

    // Collections
    let collection = [2, 3, 5, 10, 30, 50, 85, 100];
    let mut index = 0;

    while index < 8 {
        println!("the value is: {}", collection[index]);
        index += 1;
    }

    let collection = [1, 2, 3, 20, 33, 50, 89, 100, 200];

    for item in collection {
        println!("the value is: {item}");
    }

    for number in (1..100).rev() {
        println!("{number}");
    }
}
