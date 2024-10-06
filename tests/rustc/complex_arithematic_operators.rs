fn print_label(label: &str) {
    println!("{}", label);
}

fn print_binary_result(lhs: i32, op: &str, rhs: i32, result: i32) {
    println!("{} {} {} = {}", lhs, op, rhs, result);
}

fn print_unary_result(op: &str, rhs: i32, result: i32) {
    println!("{} {} = {}", op, rhs, result);
}

fn negation(value: i32) -> i32 {
    -value
}

fn add(first: i32, second: i32) -> i32 {
    first + second
}

fn subtract(first: i32, second: i32) -> i32 {
    first - second
}

fn multiply(first: i32, second: i32) -> i32 {
    first * second
}

fn divide(first: i32, second: i32) -> i32 {
    if second == 0 {
        return 0; // Handle division by zero case
    }
    first / second
}

fn modulus(first: i32, second: i32) -> i32 {
    if second == 0 {
        return 0; // Handle modulus by zero case
    }
    first % second
}

fn complex_arithmetic_tests() {
    const MAX: i32 = i32::MAX;
    const MIN: i32 = i32::MIN;

    print_label("Complex Integer Arithmetic Tests:");

    let additions = [
        (0, 1000),
        (123456789, -987654321),
        (-12345, 54321),
        (1, -1),
        (0, 0),
        (MAX, -1),
        (MIN, 1),
        (100, 200),
        (-100, -200),
        (MAX / 2, MAX / 2),
    ];

    for (a, b) in additions.iter() {
        print_binary_result(*a, "+", *b, add(*a, *b));
    }

    let subtractions = [
        (123456, 654321),
        (-999999, 999999),
        (0, 1000),
        (1000, 0),
        (-1000, -1000),
        (MAX, 1),
        (MIN, -1),
        (100, 50),
        (-100, 50),
        (MIN, MIN / 2),
    ];

    for (a, b) in subtractions.iter() {
        print_binary_result(*a, "-", *b, subtract(*a, *b));
    }

    let multiplications = [
        (0, 1000),
        (-1000, 1000),
        (-12345, 54321),
        (1, -1),
        (MAX, 1),
        (MAX, -1),
        (MIN, 1),
        (100, 0),
        (MAX / 2, 2),
        (-MAX / 2, -2),
    ];

    for (a, b) in multiplications.iter() {
        print_binary_result(*a, "*", *b, multiply(*a, *b));
    }

    let divisions = [
        (MAX, 2),
        (1000, 100),
        (-1000, 100),
        (1000, -100),
        (MAX, 1),
        (MIN, 1),
        (MAX, -1),
        // (MIN, -1),
        (100, 50),
        (-1000, 10),
    ];

    for (a, b) in divisions.iter() {
        print_binary_result(*a, "/", *b, divide(*a, *b));
    }

    let moduli = [
        (MAX, 2),
        (100, 3),
        (-100, 3),
        (MAX, -2),
        (1000, -100),
        (MIN, 2),
        (MIN, -2),
        (MAX, 3),
        (1000, 100),
        (-1000, 100),
    ];

    for (a, b) in moduli.iter() {
        print_binary_result(*a, "%", *b, modulus(*a, *b));
    }

    let negations = [
        MAX,
        0,
        1,
        -1,
        123456789,
        -123456789,
        // MIN,
        100,
        -100,
        MAX / 2,
    ];

    for a in negations.iter() {
        print_unary_result("-", *a, negation(*a));
    }
}

fn main() {
    complex_arithmetic_tests();
}
