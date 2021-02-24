use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Generate a random number between 1 and 101 (exclusive)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is {}", secret_number);
    
    loop {
        println!("\nPlease input your guess: ");

        // Create a new, mutable variable bound to a new, empty instance of a String
        //
        //  //  Remember that String retains ownership of its address in memory
        //  //  :: indicates that `new` is an associated function of the String type
        //  //  associated functions are implemented on types, not particular instances
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & indicates this is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        //println!("\nYou guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small!"),
            Ordering::Greater => println!("\nToo big!"),
            Ordering::Equal => {
                println!("\n~~~~~~~~~~\n You win!\n~~~~~~~~~~\n");
                break;
            }
        }
    }
}
