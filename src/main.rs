use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut chances = 6;
    // No mut, cannot change down the line. (const by default)
    let secret_number = rand::rng().random_range(1..=100);

    // Simple values in text: ("{value}"). Complex values in text: ("{}", value)
    println!("The computer is thinking of a number between 1 and 100, you have {chances} chances to guess it."



);

    while chances > 0 {
        println!("Please type your guess:");

        // Mutatable variable, can change down the line. Is an empty String.
        // Types are static but happen automatically.
        let mut guess = String::new();

        // Calling std::io::stdin(), made simpler by the use-statement above.
        io::stdin()
            // & = reference, mut = mutatable (refs are also const by default)
            .read_line(&mut guess)
            // .read_line returns Result, which can have Err. If so, .expect()
            // catches it, and prints the given value, instead.
            .expect("Failed to read line");

        // If something returns enum, like .parse() does, you can match and ask
        // for each case in a block!
        // Also, we already defined guess but this is a trick called shadowing.
        // More on shadowing later.
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            // Blocks are OK in match arm also!
            Err(_) => {
                println!("Please enter only digits!");
                continue;
            }
        };
        chances -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too low! {chances} chances left!"),
            Ordering::Greater => println!("{guess} is too high! {chances} chances left!"),
            Ordering::Equal => {
                println!("You got it! It's {secret_number}!");
                // Stop the final line from saying "too bad".
                chances = 1;
                break;
            }
        }
    }

    if chances == 0 {
        println!("Too bad... The number was {secret_number}.");
    }
}
