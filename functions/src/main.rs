fn main() {
    let sum = add(5, 10);
    println!("The sum of added values is: {sum}");

    let divide = divide(100, 5);
    println!("The sum of divide is: {divide}");

    say_name("Robin");

    let x = {
        let y = 2;
        y + 10
    };

    println!("Value of x is: {x}");

    let ten = ten();
    println!("{ten}");

    let numb = 230;

    // Controlflow,
    // if, else if and else
    if numb < 250 {
        println!("Number is lower than 250");
    }

    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let xy = if condition { 100 } else { 50 }; // if in a let statement
    println!("Value of xy is: {xy}");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn divide(a: i32, b: i32) -> i32 {
    return a / b;
}

fn say_name(name: &str) {
    println!("Hello, {name}");
}

fn ten() -> i32 {
    10 // returns 10
}