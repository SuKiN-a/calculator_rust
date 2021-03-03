use crate::parsed_argument::ParsedArg;

pub fn compute(parsed_arg: ParsedArg) -> f64 {
    if parsed_arg.operator == "+" {
        parsed_arg.num1 + parsed_arg.num2
    } else if parsed_arg.operator == "-" {
        parsed_arg.num1 - parsed_arg.num2
    } else if parsed_arg.operator == "*" {
        parsed_arg.num1 * parsed_arg.num2
    } else if parsed_arg.operator == "/" {
        parsed_arg.num1 / parsed_arg.num2
    } else if parsed_arg.operator == "^" {
        let mut i = 0;
        let mut product = parsed_arg.num1;
        if parsed_arg.num2 as isize == 0 {
            return 1.0;
        }
        while i + 1 < parsed_arg.num2.round() as isize {
            product *= parsed_arg.num1;
            i += 1;
        }
        product
    } else {
        panic!("invalid operation!");
    }
}
