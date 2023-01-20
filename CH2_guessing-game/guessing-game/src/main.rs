use std::io;    // import the input/output library into scope
                // std is the standard library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");  // Prompt a user with text
    
    // Generate random number.
    // rand crate has been installed and can be used
    // thread_rng instantiates a rng for this process thread
    // gen_range is imported from the Rng library above to act on the rng 
    // instance.
    let secret_number = rand::thread_rng().gen_range(1..=100);  

    // loop until breakpoint at Equal check
    loop {
        println!("Please input your guess.");
        
        // Create a new variable to store the guess.
        // We use let to  create a variable (`let apples = 5;`).
        // We also make it mutable so that it's value can change.
        // We bind the `guess` variable to a new, empty string.
        let mut guess = String::new();  

        // insatntiate standard in handler.
        // call read_line on the handler. This appends input to the `guess` 
        // string.
        // read_line returns a Result enum (type for error handling). 
        // Handle failure.
        // This is not necessary but we will get a warning at compilation if we 
        // dont do thing.
        // Write the error handlers!
        io::stdin()                     
            .read_line(&mut guess)      
            .expect("Failed to read line"); 

        // We want to convert our read guess which is a string to an int (u32).
        // We trim away whitespace and parse the value.
        // Parse returns a Result enum, which is used for error checking.
        let guess: u32 = match guess.trim().parse() {   
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");   // values can be inserted into 
                                            // strings with {}

        // compare guess to secret_number
        // The compiler is so smart that secret_number is actually infered to be
        // an u32 at assignment above because the compiler can tell that they
        // it is compared with guess.
        // This compare function returns an enum for the cases of comparison: 
        // greater than, less than, equal to.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    
}
