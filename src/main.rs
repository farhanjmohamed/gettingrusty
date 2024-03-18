#![allow(unused)]
use std::{i8, io};

fn main() {
    // printing hello world
    println!("hello world");
    // creating and printing a variable
    let x: &str = "Farhan"; // added type of string. you dont have to add type to let.
    println!("{}", x);

    // Scaler Types
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

    println!("{0} is {1} years old and his college gps was {2}. that statement is totally {3}", name, age, college_gpa, is_alive);



    //Compound Types
    // Tuple
    let tup: (i32, bool, char) = (1, true, 'f');
    let mut tup2: (i8, bool, char) = (1, true, 'f'); // the type is reliant on elements being stored in the tuple.
    tup2 = (2, false, 'a');

    println!("Tuple: {}", tup2.1);

    // array (add type of value and length)
    let arr: [i32; 5] = [5, 4, 3, 2, 1];
    let arr2: [&str; 3] = ["John", "Steve", "Nick"];
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

    println!("the sum is {}", add_numbers(5, 7));

    println!("What is your favorite cartoon?");
    let mut favorite_cartoon: String = String::new();
    io::stdin().read_line(&mut favorite_cartoon).expect("Failed to receive input!");
    println!("your favorite cartoon is {}", favorite_cartoon);

    


}

// made a function :) return type after arrow.
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

