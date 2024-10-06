pub fn print_label(label: &str) {
    println!("{}", label);
}

fn print_binary_assignment_result(lhs: &str, op: &str, rhs: &str, result: &str) {
    println!("{}", format!("{} {} {} = {}", lhs, op, rhs, result));
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

    for &(initial, rhs) in &test_cases {
        a = initial;
        a += rhs;
        print_binary_assignment_result(
            &initial.to_string(),
            "+=",
            &rhs.to_string(),
            &a.to_string(),
        );

        a = initial;
        a -= rhs;
        print_binary_assignment_result(
            &initial.to_string(),
            "-=",
            &rhs.to_string(),
            &a.to_string(),
        );

        a = initial;
        a *= rhs;
        print_binary_assignment_result(
            &initial.to_string(),
            "*=",
            &rhs.to_string(),
            &a.to_string(),
        );

        if rhs != 0 {
            a = initial;
            a /= rhs;
            print_binary_assignment_result(
                &initial.to_string(),
                "/=",
                &rhs.to_string(),
                &a.to_string(),
            );
        }

        if rhs != 0 {
            a = initial;
            a %= rhs;
            print_binary_assignment_result(
                &initial.to_string(),
                "%=",
                &rhs.to_string(),
                &a.to_string(),
            );
        }
    }
}

fn main() {
    assignment_operator_tests();
}
