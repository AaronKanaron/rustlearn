use std::io::stdin;

fn fibonacci_seq(index: i32) -> String {
    
    let mut prev: i128 = 0;
    let mut curr = 1;
    let mut next;
    let mut res = String::new();

    for i in 0..index {
        if i == 1 {
            res.push_str(&i.to_string());
            res.push(' ');
        } else{
            next = prev + curr;
            prev = curr;
            curr = next;
            
            res.push_str(&next.to_string());
            res.push(' ');
        }
    }
    return res;
}

pub fn fibonacci_run() {
    println!("Enter an index for the fibonacci sequence: ");

    let mut num_input = String::new();
    stdin().read_line(&mut num_input).expect("Failed to read.");
    let num = num_input.trim().parse().expect("Non number provided");

    println!("{}", fibonacci_seq(num));
}