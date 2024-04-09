use std::io;

fn main() {
    println!("Digite um número: ");

    let mut scan_num = String::new();

    io::stdin()
        .read_line(&mut scan_num)
        .expect("Failed to read line");

    let num: i32 = scan_num.trim().parse().expect("A number");

    fizz_buzz(num);
}

fn fizz_buzz(x: i32){
    if x % 5 == 0 && x % 3 == 0 {
        println!("FizzBuzz");
    } else if x % 5 == 0 {
        println!("Buzz");
    } else if x % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{x}");
    }
}
