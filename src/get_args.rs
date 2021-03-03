use std::env;

fn arg_parse(to_parse: &[String], value: usize) -> f64 {
    to_parse.get(value).unwrap().parse::<f64>().unwrap()
}

pub fn get_args() -> (f64, String, f64) {
    let args: Vec<String> = env::args().collect();
    let num1 = arg_parse(&args, 1);
    let num2 = arg_parse(&args, 3);
    //    println!("num1 = {}\nnum2 = {}", num1, num2)
    (num1, args.get(2).unwrap().to_string(), num2)
}
