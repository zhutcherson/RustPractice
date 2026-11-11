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
