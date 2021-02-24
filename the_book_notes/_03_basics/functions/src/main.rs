fn main() {
    println!("Hello, world!");

    // a function does not need to be defined before the line it is called
    // it simply needs to be defined somewhere
    another_function(5);
    yet_another_function(25, 6);
}

// check out the cool new concept below: function parameters!
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

// a function can have many parameters, separated by commas:
fn yet_another_function(y: i32, z:i32) {
    println!("The value of y is: {}\nThe value of z is: {}\n", y, z);
}
