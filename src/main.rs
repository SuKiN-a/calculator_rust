mod compute;
mod get_args;
mod parsed_argument;

fn main() {
    let tup = get_args::get_args();
    let parg = parsed_argument::ParsedArg {
        operator: tup.1,
        num1: tup.0,
        num2: tup.2,
    };
    println!("{} is the result!", compute::compute(parg));
}
