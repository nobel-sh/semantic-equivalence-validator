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

fn print_binary_result(lhs: &str, op: &str, rhs: &str, result: &str) {
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

fn print_unary_result(op: &str, rhs: &str, result: &str) {
    unsafe {
        fprintf(
            stdout,
            "%s %s = %s\n\0" as *const str as *const i8,
            op as *const str as *const i8,
            rhs as *const str as *const i8,
            result as *const str as *const i8,
        );
    }
}

fn bool_to_str(val: bool) -> &'static str {
    if val {
        return "true";
    }
    "false"
}

fn logical_and(lhs: bool, rhs: bool) -> bool {
    lhs && rhs
}

fn logical_or(lhs: bool, rhs: bool) -> bool {
    lhs || rhs
}

fn logical_not(val: bool) -> bool {
    !val
}

fn logical_operator_tests() {
    print_label("Logical Operator Tests:");

    let cases = [(true, true), (true, false), (false, true), (false, false)];

    let mut index = 0;
    while index < 4 {
        let (lhs, rhs) = cases[index];
        print_binary_result(
            bool_to_str(lhs),
            "&&",
            bool_to_str(rhs),
            bool_to_str(logical_and(lhs, rhs)),
        );
        index += 1;
    }

    index = 0;
    while index < 4 {
        let (lhs, rhs) = cases[index];
        print_binary_result(
            bool_to_str(lhs),
            "||",
            bool_to_str(rhs),
            bool_to_str(logical_or(lhs, rhs)),
        );
        index += 1;
    }

    let not_cases = [true, false];

    index = 0;
    while index < 2 {
        let val = not_cases[index];
        print_unary_result("!", bool_to_str(val), bool_to_str(logical_not(val)));
        index += 1;
    }
}

fn main() -> i32 {
    logical_operator_tests();
    0
}
