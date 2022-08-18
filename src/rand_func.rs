use rand::Rng;


fn rand_1() -> String {
    "This is the first function!".to_string()
}

fn rand_2() -> String {
    "Secondo!".to_string()
}

fn rand_3() -> String {
    "Tertiär!".to_string()
}

pub fn rand_func_run() {
    let num = rand::thread_rng().gen_range(0..3);

    match num {
        0 => println!("{}", rand_1()),
        1 => println!("{}", rand_2()),
        2 => println!("{}", rand_3()),
        _ => println!("Ching chong")
    }
}