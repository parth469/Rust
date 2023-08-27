fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    println!("The value of a is{a} and the value of b is {b}");
    // let c = b * a; // error will be mismatched types for any opration you want you can do with same i32 /i32 other wise error
    // * if you try to use u32 and write 5-10 then it will give error
    println!("sum of number is {} ", a + b);
    println!("sub of number is {} ", a - b);
    println!("multi of number is {} ", a * b);
    println!("div of number is {} ", a / b);

    // ! Float number
    let x: f64 = 5.6;
    let y: f64 = 10.2;

    println!("sum of number is {} ", x + y);
    println!("sub of number is {} ", x - y);
    println!("multi of number is {} ", x * y);
    println!("div of number is {} ", x / y);
}
