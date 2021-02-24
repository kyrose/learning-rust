//  Rust is a **statically typed** language
//      := variable type must be known at compile time
//  Rust is able to infer a variable's type based on the value assigned to it
//      UNLESS there are several types possible based on that value
//      ex:
//          `let guess = "42".trim().parse.expect("Not a number");`
//               ^^^^^ "42" -> an integer 42; any of the integer types are possible for this value
//

//  SCALAR TYPES

fn main() {
//  Floating-point numbers
    //  two different types for floats:
    //  1. `f64`, the default type for floats
    let x = 2.0;
    //  2. `f32`
    let y: f32 = 3.0;

    //  have lengths of 32-bits and 64-bits respectively (hopefully their names make this clear)
    //  `f32` is single precision, `f64` double
    //      NOTE: not actually sure what this means?

// Numeric Operations
    //  These should all look familiar and be self-explanatory
    //  Rust first evaluates the number on the right, then assigns it to the variable on the left

    //addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 40;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

// Booleans
    // Again, similar to how you use/declare vars of type boolean in other languages
    let t = true;
    // and here we explicitly set type:
    let f: bool = false;

// Character type
    let c = 'z';
    let z = 'ℤ';
    let my_pref_heart = '💗';

    println!("\nHere are some characters:\n  c = {}\n  z = {}\n  I can even use emojis! {}\n",
        c, z, my_pref_heart);
}
