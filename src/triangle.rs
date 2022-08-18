use std::io::stdin;

fn write_dots(height: i16) -> String {
    "*".repeat(height as usize)
}

pub fn triangle_run() {
    println!("Write the height of the triangle");

    let mut input_text = String::new();
    stdin().read_line(&mut input_text).expect("Failed to read");
    let num = input_text.trim().parse::<i16>().expect("Not a number");
    
    //To bottom
    for i in 0..num {
        println!("{}", write_dots(num - i));
    }

    //To top
    for i in 0..num + 1{
        println!("{}", write_dots(i));
    }

    //To top, skip middle row
    for i in 0..num + 1 {
        if i % 2 != 0 {
            println!("{}", write_dots(i));
        }
    }
}