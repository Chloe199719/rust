use std::io;

use rand::Rng;
fn main() {
    println!("Enter the range of the random hash");

    let mut range = String::new();
    io::stdin()
        .read_line(&mut range)
        .expect("Failed to read line");
    let int_range: i32 = range.trim().parse().unwrap();
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_";
    let mut random_hash: String = String::new();
    for _i in 0..=int_range{
        let num: i32 = rng.gen_range(0..=63);
        let c= charset.chars().nth(num as usize).unwrap();
        random_hash.push_str(&c.to_string());
 
    }
    println!("Random Hash: {}", random_hash)
}
