use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    // Перевірка дільників
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Введіть число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let number_to_check: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Неправильний ввід. Будь ласка, введіть ціле число.");
            return;
        }
    };

    if is_prime(number_to_check) {
        println!("{} є простим числом.", number_to_check);
    } else {
        println!("{} не є простим числом.", number_to_check);
    }
}
