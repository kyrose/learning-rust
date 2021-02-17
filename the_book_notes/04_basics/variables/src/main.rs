// SHADOWING
fn shadowing_examples() {
    // Every time we use the `let` keyword we are effectively creating a new variable
    //      this allows us to change the type, but use the same name
    let spaces = "    ";        // &str
    let spaces = spaces.len();  // usize
    println!("\nspaces: {}", spaces);

    // By shadowing, we can perform a few transformations on a value,
    // but ensure that the variable is immutable after the transformations are completed
    let y = 5;
    println!("\ny = {}", y);

    let y = y + 1;
    println!("y = y + 1 = {}", y);

    let y = y * 2;
    println!("y = y * 2 = {}", y);
}

// CONSTANTS
fn constants_examples() {
    // Declare a constant with the keyword `const`
    // When declaring, the type of value must be included!
    //
    // You can only set constants to constant expressions
    //      NOT to results of a function
    //      NOT to values computed at runtime
    //
    // Rust naming convention for constants is uppercase, snake case.
    
    const SPEED_OF_LIGHT: u32 = 299_792_458;
    println!("\nThe speed of light is {} m/s", SPEED_OF_LIGHT);
}

fn main() {
    let x = 5;
    println!("\nThe initial value of x is: {}", x);
    
    // Variables are immutable by default. Trying to compile with the following uncommented returns
    // error[E0384]: cannot assign twice to immutable variable `x`
    // x = 6;
    //
    // We can either make x mutable:
    //      `let mut x = 5`
    //
    // Or we can SHADOW a variable: declare a new variable with the same name as a previous var
    let x = 6;
    println!("The final value of x is: {}", x);
    
    shadowing_examples();
    constants_examples();
}
