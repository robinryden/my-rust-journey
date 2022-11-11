fn main() {
    let x = 2.0;
    println!("The value of x is: {:.1}", x);

    let a: f64 = 44.0;
    println!("The value of a is: {:.2}", a);

    // addition
    let sum = 200 + 150;
    println!("The sum is: {sum}");

    // subtraction
    let diff = 154.3 - 53.4;
    println!("The diff is: {diff}");

    // multiplication
    let product = 3939 * 60;
    println!("The product is: {product}");

    // division
    let divide = 2523.4 / 45.2;
    println!("The result of divide is: {divide}");

    let remainder = 50 % 100;
    println!("Remainder is: {remainder}");

    // boolean
    let is_valid: bool = true;

    if is_valid {
        println!("It is valid");
    } else {
        println!("Not valid");
    }

    // Char type
    let cat: char = '😻';
    println!("Interesting little cat here {cat}");

    // Compound types
    // tuples, arrays
    let tuple: (i32, f32, u8) = (150, 2.0, 5);
    println!("Our tuple with values: {:?}", tuple);

    let tuple_first_item = tuple.0;
    println!("First tuple item: {tuple_first_item}");

    let tuple_second_item = tuple.1;
    println!("Second tuple item: {tuple_second_item}");

    let tuple_third_item = tuple.2;
    println!("Third tuple item: {tuple_third_item}");

    // Arrays
    let books = ["Rust Programming", "Golang for beginners", "Java expert. After 5 years threading"];
    println!("{:?}", books);

    let arr: [i32; 3] = [1, 2, 3];
    println!("{:?}", arr);
    println!("First item in array is: {}", arr[0]);
    println!("Second item in array is: {}", arr[1]);
    println!("Third item in array is: {}", arr[2]);
}
