use std::io;

fn main() -> Result<(),std::io::Error> {
    let mut input1 = String::new();
    println!("Enter your first Number");
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read Line");
    let number: i32 = input1.trim().parse().expect("Invalid number");
    println!("This number {number}");
    Ok(())
}
