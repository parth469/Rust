fn main() {
    // Two type data -> scalar & Compound
    // * A scalar type represents a single value.
    // * integers, floating-point numbers, Booleans, and characters.

    // ! Integer Types
    // ? Two type -> unsigned(u) || signed (i)

    let x: i64 = -24; // ? signed default size u32 or i32
    let y: u8 = 23;
    // * let y: u8 = 523 error in this is it only store value upto 255
    // ? Formula to get size  = -(2n - 1) to 2n - 1 - 1

    // * if we use u unsigned then it give error example
    // let x: u64 = -24;
    println!("the value of x {x} with sign and this is with unsigned {y}");

    // ! float Types

    // ? by default f64
    let c = 4.6;
    println!("float value {c}");

    // ! Booleans Types
    let true_value = true;
    let falus_value: bool = false;

    println!("{true_value} or {falus_value} ");

    // ! characters Types
    let x: char = 'c';
    let xy: char = 's';
    let im: char = '💀';
    println!("{xy} {x} {im}")
}
