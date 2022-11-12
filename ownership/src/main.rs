fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");

    println!("{}", s);

    // Ways Variables and Data Interact: Move
    let _x = 10;
    let _y = _x;

    let _s1 = String::from("Hello");
    let _s2 = _s1;

    // println!("{}, world!", s1); // <- value borrowed here after move, error

    // Ways Variables and Data Interact: Clone
    // If we do want to deeply copy the heap data of the String
    let s3 = String::from("Hey");
    let s4 = s3.clone(); // <- heap data gets copied

    println!("s3 = {}, s4 = {}", s3, s4);

    // Stack-Only Data: Copy
    // a is still valid and wasn’t moved into b

    // The reason is that types such as integers 
    // that have a known size at compile time are stored entirely on the stack, 
    // so copies of the actual values are quick to make
    let a = 5;
    let b = a; 

    println!("a = {}, b = {}", a, b);

    // Ownership and Functions example practice
    let s5 = String::from("Hello"); // s5 comes into scope

    takes_ownership(s5);    // s5's value moves into the function...
                            // ... and so is no longer valid here

    let c = 10;             // c comes into scope

    makes_copy(c);          // c would move into the function,
                            // but i32 is Copy, so it's okay to still
                            // use c afterward
    // println!("value of s5 is: {}", s5);
    println!("value of c is: {}", c);


    // Return Values and Scope
    let _s6 = gives_ownership();     // gives_ownership moves its return
                                    // value into s6
    
    let s7 = String::from("Hello again");   // s7 comes into scope

    let s8 = takes_and_gives_back(s7);      // s7 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s8

    println!("value of s8 = {}", s8);

    // Transferring ownership of return values
    // The ownership of a variable follows the same pattern every time: 
    // assigning a value to another variable moves it

    let some_string = String::from("hello there again");
    let (item, len) = calculate_string(some_string);

    println!("The length of '{}' is {}", item, len);
}

fn calculate_string(some_string: String) -> (String, usize) {
    let length = some_string.len(); // returns the length of the string
    (some_string, length)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let something = String::from("Hello there");
    something
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}