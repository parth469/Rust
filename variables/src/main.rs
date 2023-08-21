fn main() {
    // We make variable name x with value 32
    let x = 32;
    // In this line we are just printing x value
    // * to print variable value we have to write variable name in {}
    println!(" the value of x is {x}");

    // ! variables are immutable

    // ? if you want to assigns variable x to new value it will be error
    // * x = 5;  error cannot assign twice to immutable variable `x`
    // ? to reassign ant variable you have to add mut before variable name

    // * Example :-

    let mut change_variable = 23;
    println!(" the value before change {change_variable}");
    change_variable = 6;
    println!(" the value after change {change_variable}");

    // ? const variable which are unchangeable mosley use Upper case to write and also we have to write it type like i32 i64

    // * Example :-

    const NAME: i64 = 235;
    print!("you are is {NAME}")
}

// ! Result :-
// Two type of variable let , const 
// to change variable value you have to use mut it only work with let
// while adding const you have to pass type 