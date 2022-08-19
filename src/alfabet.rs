use std::io::stdin;

pub fn alfabet_run() {
    println!("Enter a string to sort");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read");
    let input_string = input.trim().to_lowercase();

    let mut vowel_count: i8 = 0;
    let mut consonant_count: i8 = 0;
    let mut special_count: i8 = 0;

    let vowels: Vec<char> =  vec!['a', 'e', 'i', 'o', 'u', 'å', 'ä', 'ö'];   

    for i in input_string.chars() {
        if i.is_alphabetic() {
            if vowels.contains(&i) { vowel_count += 1 }
            else { consonant_count += 1 }
        } else {
            special_count += 1
        }
    }

    println!("{} vowels, {} consonants, {} specials", vowel_count, consonant_count, special_count)
}