extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut counter = 0;
    loop {
        counter += 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a positive integer!"); //数値を入力してください！

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                println!("you can find numbers in {} times!",counter);
                break;
            },                  //やったね！
        }
    }
}
