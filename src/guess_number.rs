use std::io::stdin;
use rand::Rng;


fn guess() -> i8 {
    let mut guess_input = String::new();
    stdin().read_line(&mut guess_input).expect("Failed to read");
    let guess = guess_input.trim().parse::<i8>().expect("Not a number");
    
    guess
}    

fn restart(answer: i8) {
    let guess_num = guess();

    if guess_num == answer { println!("You win!") }
    else if guess_num < answer {
        println!("Too low, try again");
        restart(answer);
    }
    else {
        println!("Too high, try again");
        restart(answer);
    }
}

pub fn guess_number_run() {
    println!("Guess a number: ");
    let answer: i8 = rand::thread_rng().gen_range(1..101);
    restart(answer);    
}