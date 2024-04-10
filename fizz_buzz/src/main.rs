use std::io;

fn main() {
    println!("Digite um número: ");

    let mut scan_num = String::new();

    io::stdin()
        .read_line(&mut scan_num)
        .expect("Failed to read line");

    let num: i32 = scan_num.trim().parse().expect("A number");

    println!("{}", fizz_buzz(num));
}

fn fizz_buzz(x: i32) -> String {
    if x % 5 == 0 && x % 3 == 0 {
        return "FizzBuzz".to_string();
    } else if x % 5 == 0 {
        return "Buzz".to_string();
    } else if x % 3 == 0 {
        return "Fizz".to_string();
    } else {
        return x.to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::fizz_buzz;

    #[test]
    fn fizz() {
        assert_eq!("Fizz", fizz_buzz(3));
    }

    #[test]
    fn buzz() {
        assert_eq!("Buzz", fizz_buzz(5));
    }

    #[test]
    fn fizzbuzz() {
        assert_eq!("FizzBuzz", fizz_buzz(15));
    }

    #[test]
    fn num() {
        assert_eq!("7", fizz_buzz(7))
    }
}
