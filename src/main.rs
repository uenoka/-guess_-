use std::io;

fn main() {
    println!("guess the number !");
    println!("please input your guess !");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");
    println!("you guessed : {}",guess);
}
