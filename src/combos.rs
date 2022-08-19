// List all possible combinations of letters in a 4-letter word. Eg 'TEST' can be unscrambled as TEST, TETS, TSET, TSTE, TTSE, TTES, etc.
use std::io::stdin;

fn premutations(word: String) -> Vec<String> {

    if word.len() <= 1 {
        return vec![word];
    }

    let trimmed = word.chars().skip(1).collect();

    let perms = premutations(trimmed);
    let current_char = word.chars().nth(0).unwrap();
    let mut result = Vec::new();

    for perm in &perms {
        for i in 0..&perms.len() + 1 {
            let front: String = perm.chars().take(i).collect();
            let rest: String = perm.chars().skip(i).collect();
            result.push(format!("{}{}{}", front, current_char, rest));
        }
    }

    result
}

pub fn combos_run() {
    //ask for string
    println!("Enter a word (max 6 letters): ");
    let mut word_input = String::new();
    stdin().read_line(&mut word_input).expect("Failed to read");
    let word = word_input.trim().to_string();

    if word.len() > 6 {
        panic!("String too long, takes too long to compute")
    }

    let total = premutations(word);
    let mut unique = total.clone();

    unique.sort();
    unique.dedup();

    // println!("Permutations: \n{:?}\n", total);
    println!("Distinct permutations: \n{:?}\n", unique);
    // println!("Total permutations: {}", total.len());
    println!("Distinct permutations: {}", unique.len());
}