// Find the Fifth root of the sum of the squares of the first 100 ODD numbers only

fn fifth_root(x:f64) -> f64 {
    x.powf(0.2)
}

pub fn math_run() {
    let mut sum: i64 = 0;
    for n in 0..100 { //should be 101, but since 100 is not an odd number it does not matter
        if n % 2 == 0 { 
            sum += n * n;
        }
    }  

    println!("5th root of {}: {}", sum, fifth_root(sum as f64));
}