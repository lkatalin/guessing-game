use std::io;
use std::cmp::Ordering;

// Rng is a trait in the rnd crate that defines methods for
// random number generators to implement
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // rand::thread_rng specifies the rng we want to use;
    // thread_rng is a function in rnd;
    // we call its gen_range method, defined by the Rng trait
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // makes guess a mutable variable
        let mut guess = String::new();

        // could be written as std::io::stdin
        // std::io::stdin returns an instance of std::io::Stdin
        //
        // &mut guess makes the reference to guess also mutable
        //
        // .read_line returns a value: number of bytes of input
        //
        // .expect crashes and displays text in case .read_line 
        // returns an Err value; otherwise returns Ok value;
        // so EXPECT returns either an error or the result of
        // READ_LINE, which is the number of bytes in the input

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing reuses 'guess' var name for this conversion;
        // .trim gets rid of \n that is otherwise present
        // in the string version of guess;
        // string's .parse parses string into number
        //
        // Rust will also infer that secret_number is a u32 because
        // of the comparison that comes later!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please print a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // .cmp method returns a variant of Ordering, 
        // ex. Ordering::Greater
        //
        // match stops looking after a match is found
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
