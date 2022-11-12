fn main() {
    // References and Borrowing
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);    // <- reference to memory address
                                        // ..without taking ownership of s1

    println!("The length of '{}' is {}", s1, len);

    // modify something we’re borrowing
    // Mutable References
    let mut s2 = String::from("Hello");

    change(&mut s2); // mutable reference

    println!("{}", s2); // now prints Hello, world after change on memory

    // The Slice Type
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) { // accept a mutable reference
    some_string.push_str(", world");  // do the actual change
} 
