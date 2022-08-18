use std::io;

/*
Does not work with negative numbers, numbers that are too big, decimals
*/


fn factorial(num: i64) -> i64 {
    
    if num <= 1 {
        return 1;
    }

    num * factorial(num - 1)
}


pub fn factorial_run() {
    println!("Enter a number: ");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");
    
    let num = input_text.trim().parse::<i64>().expect("Not a number");

    println!("{}", factorial(num))
}