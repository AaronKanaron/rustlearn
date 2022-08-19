use std::io::stdin;

//Memoized fibonacci algr
fn fib(n: i128) -> i128 {
    if n < 2 {
        1
    } else {
        let mut memo = [1, 1];
        let mut n = n - 2;

        loop {
            let [a, b] = memo;
            let c = a + b;

            if n == 0 {
                return c;
            }

            memo = [b, c];
            n -= 1;
        }
    }
}

pub fn fibonacci_run() {
    println!("Enter an index for the fibonacci sequence: ");

    let mut num_input = String::new();
    stdin().read_line(&mut num_input).expect("Failed to read.");
    let num = num_input.trim().parse::<i128>().expect("Non number provided");

    println!("{}", fib(num-1));
}