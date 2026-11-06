use std::io;
// use RustPractice::add;

fn main() {
    println!("\n\nProvide two numbers and an operator to do some cool math!");
    println!("Use the format: [num1] [operator] [num2]");

    // create a storage variable for holding user input
    // currently empty
    let mut user_input = String::new();

    // stop the program to take in user input, put it in user_input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read line!");

    // remove white space before and after user input
    let user_input = user_input.trim();

    let input_vector: Vec<&str> = user_input.rsplit(' ').collect();

    if input_vector.len() != 3{
        println!("You must provide 3 arguments like: [num1] [operator] [num2]");
        return
    }

    let first_num: i32 = match input_vector[0].parse(){
        Ok(num) => num,
        Err(_) => {
            println!("The first parameter must be a number!");
            0
        },
    };
    if input_vector[1].len() != 1 {
        println!("The second parameter must be a function!");
        return
    }
    let operator: &str = input_vector[1];
    let second_num: i32 = match input_vector[2].parse(){
        Ok(num) => num,
        Err(_) => {
            println!("The third parameter must be a number!");
            0
        }
    };

    let mut output: i32;

    match operator {
        // "+" => output = add(),
        "+" => println!("ADD"),
        &_ => println!("Invalid function provided!"),
    }

}
// comment
// ^ this is a dumb comment
// that was a smart comment
// testing