fn print_label(label: &str) {
    println!("{}", label);
}

fn print_binary_result(lhs: i32, op: &str, rhs: i32, result: i32) {
    println!("{}", format!("{} {} {} = {}", lhs, op, rhs, result));
}

fn print_unary_result(op: &str, rhs: i32, result: i32) {
    println!("{}", format!("{} {} = {}", op, rhs, result));
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

    for (a, b) in additions {
        print_binary_result(a, "+", b, add(a, b));
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

    for (a, b) in subtractions {
        print_binary_result(a, "-", b, subtract(a, b));
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

    for (a, b) in multiplications {
        print_binary_result(a, "*", b, multiply(a, b));
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

    for (a, b) in divisions {
        print_binary_result(a, "/", b, divide(a, b));
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

    for (a, b) in moduli {
        print_binary_result(a, "%", b, modulus(a, b));
    }

    let negation_cases = [1, -1, 0, 100, -100];

    for a in negation_cases {
        print_unary_result("-", a, negation(a));
    }
}

fn main() {
    simple_arithmetic_tests();
}
