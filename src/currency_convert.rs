#![allow(
    unused_imports,
    dead_code,
    unused_variables
)]

use std::{io::stdin, convert};

const SEK_TO_USD: f32 = 0.095;
const USD_TO_SEK: f32 = 10.5;

enum Choice { SEK, USD }

fn convert(amount: f32, choice: Choice) -> f32 {
    match choice {
        Choice::SEK => amount * SEK_TO_USD,
        Choice::USD => amount * USD_TO_SEK
    }
}

pub fn currency_convert_run() {
    println!("Chose from the following (type number):\n(1) SEK to USD\n(2) USD to SEK");

    let mut input_text = String::new();
    stdin().read_line(&mut input_text).expect("Failed to read");
    let num = input_text.trim().parse::<i64>().expect("Non number provided");

    let choice;
    let sign;
    let converted_sign;

    match num {
        1 => {
            choice = Choice::SEK;
            sign = "kr";
            converted_sign = "$";
        },
        2 => {
            choice = Choice::USD;
            sign = "$";
            converted_sign = "kr";
        },

        _ => {
            println!("Not an option from the list");
            return;
        }
    };

    println!("Enter amount to convert: ");
    let mut amount_in = String::new();
    stdin().read_line(&mut amount_in).expect("Failed to read");

    let amount = amount_in.trim().parse::<f32>().expect("Non number provided");

    println!("{}{} is {}{:.2}", amount, sign, convert(amount,choice), converted_sign)

}


