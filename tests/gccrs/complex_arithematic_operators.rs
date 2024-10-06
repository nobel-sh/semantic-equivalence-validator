extern "C" {
    fn fprintf(stream: *mut i8, format: *const i8, ...);
    static stdout: *mut i8;
}

fn print_label(label: &str) {
    unsafe {
        fprintf(
            stdout,
            "%s\n\0" as *const str as *const i8,
            label as *const str as *const i8,
        );
    }
}

fn print_binary_result(lhs: i32, op: &str, rhs: i32, result: i32) {
    unsafe {
        fprintf(
            stdout,
            "%d %s %d = %d\n\0" as *const str as *const i8,
            lhs,
            op as *const str as *const i8,
            rhs,
            result,
        );
    }
}

fn print_unary_result(op: &str, rhs: i32, result: i32) {
    unsafe {
        fprintf(
            stdout,
            "%s %d = %d\n\0" as *const str as *const i8,
            op as *const str as *const i8,
            rhs,
            result,
        );
    }
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
    first / second
}

fn modulus(first: i32, second: i32) -> i32 {
    first % second
}

fn complex_arithmetic_tests() {
    const MAX: i32 = 2147483647;
    const MIN: i32 = -2147483648;

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

    let mut index = 0;
    while index < 10 {
        let (a, b) = additions[index];
        print_binary_result(a, "+", b, add(a, b));
        index += 1;
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

    let mut index = 0;
    while index < 10 {
        let (a, b) = subtractions[index];
        print_binary_result(a, "-", b, subtract(a, b));
        index += 1;
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

    let mut index = 0;
    while index < 10 {
        let (a, b) = multiplications[index];
        print_binary_result(a, "*", b, multiply(a, b));
        index += 1;
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

    let mut index = 0;
    while index < 9 {
        let (a, b) = divisions[index];
        print_binary_result(a, "/", b, divide(a, b));
        index += 1;
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

    let mut index = 0;
    while index < 10 {
        let (a, b) = moduli[index];
        print_binary_result(a, "%", b, modulus(a, b));
        index += 1;
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

    let mut index = 0;
    while index < 9 {
        let a = negations[index];
        print_unary_result("-", a, negation(a));
        index += 1;
    }
}

fn main() -> i32 {
    complex_arithmetic_tests();
    0
}
