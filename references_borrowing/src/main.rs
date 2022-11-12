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
    // A slice is a kind of reference, so it does not have ownership.
    let mut s = String::from("hello world");

    let _word = first_word(&s);

    s.clear();

    // println!("{}", word);

    // Slices
    let w = String::from("hello there world");

    let hello = &w[0..5]; // also [..5] works 
    let there = &w[6..11];
    let world = &w[12..17]; // also [12..] works, index 12 to the end of index

    println!("first slice: {}, second slice: {}, third slice: {}", hello, there, world);

    // other slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[2..5];

    assert_eq!(slice, &[3, 4, 5]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) { // accept a mutable reference
    some_string.push_str(", world");  // do the actual change
} 
