use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main()
{
    // Game related display messages
    println!("Guess the number!");

    // The Rand library isn't standard, has to be an added crate
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop
    {
        println!("Please input your guess.");

        // Create String type variable to read the user input into
        // Variables are immutable by default, so make it mutable since we want to modify it
        let mut guess = String::new();

        // References are also immutable by default, so &mut is needed rather than just &guess
        // The read_line method returns an enumerated type for potential errors, thus expect()
        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        // Convert the passed in value (String) to an unsigned integer, error allows retry
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Cmp returns the ordering as an enumerated type, compare the guess and secret number
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>
            {
                println!("You win!");
                break;
            }
        }
    }
}
