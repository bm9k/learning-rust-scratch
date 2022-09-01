use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Did not receive input");
    println!("Hello, {}, nice to meet you!", name.trim_end());

    const ONE_MIL: u32 = 1_000_000;
    // shadowing
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age += 1;
    // subtle much?
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
