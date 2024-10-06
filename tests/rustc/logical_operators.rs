pub fn print_label(label: &str) {
    println!("{}", label);
}

fn print_binary_result(lhs: &str, op: &str, rhs: &str, result: &str) {
    println!("{}", format!("{} {} {} = {}", lhs, op, rhs, result));
}

fn print_unary_result(op: &str, rhs: &str, result: &str) {
    println!("{}", format!("{} {} = {}", op, rhs, result));
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

fn main() {
    logical_operator_tests();
}
