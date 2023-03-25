use std::io;

fn main() {
    println!("Введите число:");
    let mut base = String::new();

    io::stdin()
        .read_line(&mut base)
        .expect("Не удалось прочитать строку");

    let base: i32 = base.trim().parse().expect("Ожидалось целое число");

    println!("Введите степень:");
    let mut exponent = String::new();

    io::stdin()
        .read_line(&mut exponent)
        .expect("Не удалось прочитать строку");

    let exponent: u32 = exponent.trim().parse().expect("Ожидалось неотрицательное целое число");

    let result = pow(base, exponent);

    println!("{} в степени {} = {}", base, exponent, result);
}

fn pow(base: i32, exponent: u32) -> i32 {
    if exponent == 0 {
        return 1;
    }

    let mut result = 1;

    for _ in 1..=exponent {
        result *= base;
    }

    result
}
