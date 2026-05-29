use std::env;
use std::time::Instant;

fn is_prime(number: u32) -> bool {
    if number < 2 {
        return false;
    }
    if number == 2 {
        return true;
    }
    if number % 2 == 0 {
        return false;
    }

    let mut divisor = 3;
    while divisor * divisor <= number {
        if number % divisor == 0 {
            return false;
        }
        divisor += 2;
    }

    true
}

fn count_primes(limit: u32) -> u32 {
    let mut count = 0;
    let mut number = 2;

    while number <= limit {
        if is_prime(number) {
            count += 1;
        }
        number += 1;
    }

    count
}

fn main() {
    let limit = env::var("LIMIT")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(100_000);

    let started_at = Instant::now();
    let result = count_primes(limit);
    let elapsed = started_at.elapsed();

    println!("Rust-приложение в Docker");
    println!("Задача: подсчет простых чисел от 2 до {limit}");
    println!("Результат: {result}");
    println!("Время выполнения: {:.6} сек.", elapsed.as_secs_f64());
}
