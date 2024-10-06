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

fn print_binary_result_str(lhs: &str, op: &str, rhs: &str, result: &str) {
    unsafe {
        fprintf(
            stdout,
            "%s %s %s = %s\n\0" as *const str as *const i8,
            lhs as *const str as *const i8,
            op as *const str as *const i8,
            rhs as *const str as *const i8,
            result as *const str as *const i8,
        );
    }
}

fn print_binary_result_int(lhs: i32, op: &str, rhs: i32, result: i32) {
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

fn print_binary_result_bool(lhs: i32, op: &str, rhs: i32, result: bool) {
    unsafe {
        fprintf(
            stdout,
            "%d %s %d = %s\n\0" as *const str as *const i8,
            lhs,
            op as *const str as *const i8,
            rhs,
            bool_to_str(result) as *const str as *const i8,
        );
    }
}

fn print_unary_result_int(op: &str, rhs: i32, result: i32) {
    unsafe {
        fprintf(
            stdout,
            "%s%d = %d\n\0" as *const str as *const i8,
            op as *const str as *const i8,
            rhs,
            result,
        );
    }
}

fn bool_to_str(val: bool) -> &'static str {
    if val {
        "true"
    } else {
        "false"
    }
}

fn bitwise_and(a: i32, b: i32) -> i32 {
    a & b
}

fn bitwise_or(a: i32, b: i32) -> i32 {
    a | b
}

fn bitwise_xor(a: i32, b: i32) -> i32 {
    a ^ b
}

fn bitwise_not(a: i32) -> i32 {
    !a
}

fn left_shift(a: i32, b: i32) -> i32 {
    a << b
}

fn right_shift(a: i32, b: i32) -> i32 {
    a >> b
}

fn equal(a: i32, b: i32) -> bool {
    a == b
}

fn not_equal(a: i32, b: i32) -> bool {
    a != b
}

fn less_than(a: i32, b: i32) -> bool {
    a < b
}

fn less_than_or_equal(a: i32, b: i32) -> bool {
    a <= b
}

fn greater_than(a: i32, b: i32) -> bool {
    a > b
}

fn greater_than_or_equal(a: i32, b: i32) -> bool {
    a >= b
}

fn bitwise_operator_tests() {
    print_label("Bitwise Operator Tests:");
    let bitwise_cases = [
        (0b1010, 0b1100),
        (0b1111, 0b0000),
        (0b1010_1010, 0b0101_0101),
        (0, -1),
        (-1, -1),
        (0b1111_0000, 0b0000_1111),
        (0b1010_1010_1010_1010, 0b0101_0101_0101_0101),
        (0b1111_1111_0000_0000, 0b0000_0000_1111_1111),
        (0b1, 0b10),
        (0b11, 0b101),
        (0b111, 0b1001),
        (0b1111, 0b10001),
    ];
    let mut index = 0;
    while index < 12 {
        let (a, b) = bitwise_cases[index];
        print_binary_result_int(a, "&", b, bitwise_and(a, b));
        print_binary_result_int(a, "|", b, bitwise_or(a, b));
        print_binary_result_int(a, "^", b, bitwise_xor(a, b));
        index += 1;
    }

    let shift_cases = [
        (0b1010, 1),
        (0b1010, 2),
        (0b1010_1010, 4),
        (0b1111_0000, 8),
        (0b1, 31),
        (0b10, 30),
        (0b100, 29),
        (0b1000, 28),
        (-1, 1),
        (-2, 30),
        (-4, 29),
        (-8, 28),
    ];
    index = 0;
    while index < 12 {
        let (a, b) = shift_cases[index];
        print_binary_result_int(a, "<<", b, left_shift(a, b));
        print_binary_result_int(a, ">>", b, right_shift(a, b));
        index += 1;
    }

    let not_cases = [
        0,
        -1,
        0b1010_1010,
        0b1111_0000_1111_0000,
        0b0000_1111_0000_1111,
        1,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        -2,
        -4,
        -8,
        -16,
        -32,
        -64,
        -128,
    ];

    index = 0;
    while index < 20 {
        let a = not_cases[index];
        print_unary_result_int("~", a, bitwise_not(a));
        index += 1;
    }
}

fn comparison_operator_tests() {
    print_label("Comparison Operator Tests:");
    let comparison_cases = [
        (0, 0),
        (1, 1),
        (1, 2),
        (2, 1),
        (-1, 1),
        (100, -100),
        (-100, 100),
        (-1, -2),
        (2, -1),
        (1000, 999),
        (999, 1000),
        (50, 50),
        (-50, -50),
        (123456, 123456),
        (-123456, -123456),
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1),
    ];
    let mut index = 0;
    while index < 19 {
        let (a, b) = comparison_cases[index];
        print_binary_result_bool(a, "==", b, equal(a, b));
        print_binary_result_bool(a, "!=", b, not_equal(a, b));
        print_binary_result_bool(a, "<", b, less_than(a, b));
        print_binary_result_bool(a, "<=", b, less_than_or_equal(a, b));
        print_binary_result_bool(a, ">", b, greater_than(a, b));
        print_binary_result_bool(a, ">=", b, greater_than_or_equal(a, b));
        index += 1;
    }
}

fn main() -> i32 {
    bitwise_operator_tests();
    comparison_operator_tests();
    0
}
