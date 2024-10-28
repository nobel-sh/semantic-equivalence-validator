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

fn simple_arithmetic_tests() {
    print_label("Simple Arithmetic Tests:");

    let additions = [
        (1, 1),
        (2, 2),
        (3, 5),
        (10, 20),
        (0, 0),
        (5, -3),
        (-5, 3),
        (100, 200),
        (-100, -200),
        (0, 100),
        (50, 50),
        (100, -50),
        (-100, 100),
    ];

    let mut index = 0;
    while index < 13 {
        let (a, b) = additions[index];
        print_binary_result(a, "+", b, add(a, b));
        index += 1;
    }

    let subtractions = [
        (5, 3),
        (3, 5),
        (0, 0),
        (100, 50),
        (50, 50),
        (-5, -5),
        (-10, -5),
        (5, 10),
        (10, 0),
        (1, 0),
        (0, 1),
    ];

    let mut index = 0;
    while index < 11 {
        let (a, b) = subtractions[index];
        print_binary_result(a, "-", b, subtract(a, b));
        index += 1;
    }

    let multiplications = [
        (5, 3),
        (0, 5),
        (1, 100),
        (-5, 3),
        (-1, -1),
        (100, 100),
        (1024, 1024),
        (1000, 0),
        (10, -10),
        (-10, 10),
        (-2, -2),
    ];

    let mut index = 0;
    while index < 11 {
        let (a, b) = multiplications[index];
        print_binary_result(a, "*", b, multiply(a, b));
        index += 1;
    }

    let divisions = [
        (10, 2),
        (5, 5),
        (100, 10),
        (2147483647, 1),
        (-10, 2),
        (10, -2),
        (0, 1),
        (-100, -10),
        (100, -5),
    ];

    let mut index = 0;
    while index < 9 {
        let (a, b) = divisions[index];
        print_binary_result(a, "/", b, divide(a, b));
        index += 1;
    }

    let moduli = [
        (10, 3),
        (20, 7),
        (7, 3),
        (0, 1),
        (5, 5),
        (-10, 3),
        (2147483647, 3),
        (-20, -3),
    ];

    let mut index = 0;
    while index < 8 {
        let (a, b) = moduli[index];
        print_binary_result(a, "%", b, modulus(a, b));
        index += 1;
    }

    let negation_cases = [1, -1, 0, 100, -100];

    let mut index = 0;
    while index < 5 {
        let a = negation_cases[index];
        print_unary_result("-", a, negation(a));
        index += 1;
    }
}

fn main() -> i32 {
    simple_arithmetic_tests();
    0
}
