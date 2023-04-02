use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("secret_number:{}", secret_number);

    loop {
        println!("Please input a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("can not read_line");

        println!("The number is: {}", guess);

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win.");
                break;
            },
        }
    }

}


