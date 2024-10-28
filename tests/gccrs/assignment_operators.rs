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

fn print_binary_assignment_result(lhs: i32, op: &str, rhs: i32, result: i32) {
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

fn assignment_operator_tests() {
    print_label("Assignment Operator Tests:");

    let mut a: i32;

    let test_cases = [
        (10, 5),
        (100, 20),
        (50, 25),
        (-1, 5),
        (0, 3),
        (200, -50),
        (25, -5),
        (-10, 2),
    ];

    let mut index = 0;
    let size = 8;

    while index < size {
        let (initial, rhs) = test_cases[index];

        a = initial;
        a += rhs;
        print_binary_assignment_result(initial, "+=", rhs, a);

        a = initial;
        a -= rhs;
        print_binary_assignment_result(initial, "-=", rhs, a);

        a = initial;
        a *= rhs;
        print_binary_assignment_result(initial, "*=", rhs, a);

        if rhs != 0 {
            a = initial;
            a /= rhs;
            print_binary_assignment_result(initial, "/=", rhs, a);
        }

        if rhs != 0 {
            a = initial;
            a %= rhs;
            print_binary_assignment_result(initial, "%=", rhs, a);
        }

        index += 1;
    }
}

fn main() -> i32 {
    assignment_operator_tests();
    0
}
