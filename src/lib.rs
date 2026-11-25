use core::num;


///Takes in a string made of digits and converts it into a vector
/// of digits
pub fn vectorize(text: &str) -> Vec<u8>{
    let mut new_vec: Vec<u8> = Vec::new();
    for i in text.chars(){
        let dig = match i.to_digit(10) {
            Some(value) => value as u8,
            None => {
                println!("Non-numeric character provided!");
                0
            },
        };
        new_vec.push(dig);
        // println!("u8: {i} {}", i as u8);
    }
    new_vec
}

pub fn stringerize(num: &Vec<u8>) -> String{
    let mut output_string: String = String::new();
    
    for i in 0..num.len(){
        let dig = char::from(b'0' + num[i]);
        output_string.insert(i, dig);
    }
    output_string
}

///Takes in two vectors of digits and adds each of them together.
pub fn add_infinite(num_1: &Vec<u8>, num_2: &Vec<u8>) -> Vec<u8>{
    
    // find the smallest length between the two numbers
    let smaller_num;
    let bigger_num;

    if num_1.len() < num_2.len(){
        smaller_num = num_1;
        bigger_num = num_2;
    }
    else {
        smaller_num = num_2;
        bigger_num = num_1;
    }

    let mut new_vec: Vec<u8> = Vec::new();
    let mut carryover = 0;

    for i in (0..bigger_num.len()).rev(){
        // smaller num needs to be offset from the front (the number 2 becomes 002 when working with 123)
        let small_diff = bigger_num.len() - smaller_num.len();
        let small_dig;
        if small_diff > i {
            // prevent an overflow error
            small_dig = 0;
        } else {
            small_dig = smaller_num[i - small_diff];
        }

        let mut new_dig = carryover + small_dig + bigger_num[i];

        if new_dig > 9 {
            carryover = new_dig / 10;
            new_dig = new_dig % 10;
        }
        else {
            carryover = 0;
        }

        new_vec.push(new_dig)
    }

    if carryover > 0 {
        new_vec.push(carryover);
    }

    new_vec.reverse();
    new_vec
}

/// repeats the function `fun` `count` times on the base `num`
fn _repeat_times(num: &Vec<u8>, count: &Vec<u8>, fun: fn(&Vec<u8>,&Vec<u8>)->Vec<u8>, start: u8) -> Vec<u8> {
    
    let mut total: Vec<u8> = Vec::new();
    total.push(start);

    for i in 0..count.len() {
        // converts a digit into the full representation of that digit
        // the 3 in 4321 would be 300
        let tens: i32 = (10_i32.pow(i as u32)) * count[count.len() - i - 1] as i32;

        for _j in 0..tens{
            let curr_total = stringerize(&total);
            println!("Current total: {curr_total}");

            total = fun(&total, &num);
        }
    }

    // if count.len() == 1 && count.get(0) == 1 {

    // }

    total
}


/// takes in two number vectors and gives the multiplication product as a result
pub fn multiply_infinite(num_1: &Vec<u8>, num_2: &Vec<u8>) -> Vec<u8> {
    _repeat_times(num_1, num_2, add_infinite, 0)
}

/// takes in a base and multiplies it by itself power times
pub fn exponent_infinite(base: &Vec<u8>, power: &Vec<u8>) -> Vec<u8> {
    _repeat_times(base, power, multiply_infinite, 1)
}

/// checks if num_1 is greater than num_2
pub fn greater_than_infinite(num_1: &Vec<u8>, num_2: &Vec<u8>) -> bool {
    if num_1.len() > num_2.len() {
        return true;
    } 
    else if num_1.len() == num_2.len() {
        if num_1[0] > num_2[0] {
            return true;
        }
        else if num_1[0] == num_2[0] && num_1.len() == 1 {
            return true;
        }
        else if num_1[0] == num_2[0] && num_1.len() > 1 {
            let slice_1 = num_1[1..num_1.len()].to_vec();
            let slice_2 = num_2[1..num_2.len()].to_vec();
            return greater_than_infinite(&slice_1, &slice_2);
        }
    }

    false
}

/// determines if num_1 is less than num_2
pub fn less_than_infinite(num_1: &Vec<u8>, num_2: &Vec<u8>) -> bool {
    !greater_than_infinite(num_1, num_2) && !equal_to_infinite(num_1, num_2) 
}

/// finds if two numbers are equal
pub fn equal_to_infinite(num_1: &Vec<u8>, num_2: &Vec<u8>) -> bool{
    if num_1.len() != num_2.len() {
        return false;
    }

    for i in 0..num_1.len() {
        if num_1[i] != num_2[i]{
            return false;
        }
    }

    true
}

/// adds 2 numbers
pub fn add(a:i32, b:i32) -> i32{
    return a + b;
}

pub fn subtract(a:i32, b:i32) -> i32{
    return a - b;
}

pub fn multiply(a:i32, b:i32) -> i32{
    return a * b;
}

pub fn divide(a:i32, b:i32) -> i32{
    return a / b;
}

pub fn exponent(a:i32, b:i32) -> i32{
    let mut ans = 1;
    for _i in 0..b {
        ans = ans * a;
    }
    return ans;
}
