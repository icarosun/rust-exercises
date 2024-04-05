use std::io;

fn main() {
    println!("Digite o número: ");

    let mut scan_num_a = String::new();
    let mut scan_num_b = String::new();
    
    io::stdin()
        .read_line(&mut scan_num_a)
        .expect("Failed to read line");

    println!("Digite o segundo número: ");

    io::stdin()
        .read_line(&mut scan_num_b)
        .expect("Failed to read line");

    let num_a: u32 = scan_num_a.trim().parse().expect("A number");
    let num_b: u32 = scan_num_b.trim().parse().expect("A number");

    let sum = num_a + num_b;

    println!("A soma dos números é {sum}");
}
