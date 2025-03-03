fn main() {
    println!("Hello, world!");

    // declaring variables

    // immutable variable means not changeable variables

    let age = 24;

    // age = age + 1;

    // this will give an error

    println!("The value of age is: {}", age);

    let mut x = 1;
    x = x + 1;
    println!("The value of x is: {}", x);

    // mutable variable
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // let x = 5;
    // println!("The value of x is: {}", x);

    // let x = x + 1;
    // println!("The value of x is: {}", x);

    // immutable variable
    // let y = 5;
    // println!("The value of y is: {}", y);
    // y = 6; // this will give an error
    // let y = 5;
    // println!("The value of y is: {}", y);

    // shadowing
    // let x = 5;
    // let x = x + 1;
    // println!("The value of x is: {}", x);

    // let y = 5;
    // let y = y + 1;
    // println!("The value of y is: {}", y);

    // constants
    // const MAX_POINTS: u32 = 100;
    // println!("Maximum points is: {}", MAX_POINTS);
    // // MAX_POINTS = 101; // this will give an error
    // println!("The value of y is: {}", y);
}
