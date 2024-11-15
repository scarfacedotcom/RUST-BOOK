fn main() {
    println!("Hello, world!");
    my_fn("This is my function");
    let str = "Function call with a variable";
    my_fn(str);

    let answer = multiplication(2, 4);
    println!("the answer is {answer}");

    let result = basic_math(10, 2);
    let (multiplication, addition, substraction) = basic_math(2, 6);

    let full_name = {
        let first_name = "peter";
        let last_name = "adaaku";
        format!("{first_name}{last_name}")
    };
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication(num_1: i32, num_2: i32) -> i32 {
    println!("Computing Multiplication");
    num_1 * num_2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
