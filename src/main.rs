#![allow(unused)]

fn main() {
    // printing hello world
    println!("hello world");
    // creating and printing a variable
    let x: &str = "Farhan"; // added type of string. you dont have to add type to let.
    println!("{}", x);

    // Data Types
    //Strings
    let name: &str = "Farhan";
    // char (single quotes only)
    let first_letter: char = 'a';
    // assigned integers (can include negative numbers: range of values starts from negative and goes positive)
    let negative_age: i32 = -30;
    // unassigned (can only by positive. range of number starts at zero and goes up)
    let age: u32 = 30;
    // floats (default is f64)
    let college_gpa: f32 = 3.1;
    // boolean
    let is_alive: bool = true;
    let is_dead: bool = false;
    // array (add type of value and length)
    let arr: [i32; 5] = [5, 4, 3, 2, 1];
    // mutable variable (rust variables are immutable so we need to make it mutable by adding mut keyword to variables)
    let mut i: i32 = 0;
    //while loop showing mutablility of variable
    while i < 10 {
        i += 1;
        println!("{}", i);
    }

    // Name Shadowing (strange concept) lots to understand still. (using let makes you redefine the variable but after its defined if you dont use let you will get error unless mutable)
    let y = 4;
    println!("{}", y);

    // when changing in a scope it only changes within that scope and no where else
    {
        let y = y - 4;
        println!("{}", y);
    }

    let y = y + 1;
    println!("{}", y);

    // constant value (the convention is different and some rules to follow)
    const SECONDS_IN_MINUTES: u32 = 60; // all caps for const and snake case for naming. must provide type for all consts.
    println!("{}", SECONDS_IN_MINUTES);
}
