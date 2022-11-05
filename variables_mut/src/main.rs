fn main() {
    // variables
    // immutable as default
    let x = 5;
    println!("Value of x is: {x}");

    // mutable variable using mut
    let mut y = 10;
    println!("Value of y is: {y}");

    y = 5;
    println!("Value of y is now: {y}");

    // constants
    const AGE: u32 = 25;
    println!("Value of const age is: {AGE}");

    // shadowing, 
    // declaring new variable using the same name as previous variable
    let x = x + 50;

    {
        let x = x + 20;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");
}
