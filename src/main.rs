fn main() {
    println!("Hello, world!");

    // declaring variables

    // immutable variable means not changeable variables

    let age = 24;

    // age = age + 1;

    // this will give an error

    println!("The value of age is: {}", age);

    let mut x = 1;
    println!("The value of x before: {}", x);
    x = x + 1;
    println!("The value of x after: {}", x);

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

    // data types
    // there are four scalar types: integers, floating-point numbers, Booleans, and characters

    // integer types
    //    // let age = 42; // default integer type is i32
    // check the type of the age value
    // println!("The type of age is: {}", std::any::type_name_of_val(&age));

    // conditional statements

    // let x: i32 = 5;
    // if x < 5 {
    //     println!("x is less than 5");
    // } else if x == 5 {
    //     println!("x is equal to 5");
    // } else {
    //     println!("x is greater than 5");
    // }

    firstfunctions();
    secondfunparam(5);
    thirdfunction(5, 6);
    practise_stringliteral();

    tuple_practice();

    let num1: u8 = 5;

    let result: u8 = withreturntype(num1);
    println!("The result is: {}", result);
}

// first function without paramameters
fn firstfunctions() {
    println!("Hello, from the first function!");
}

// remember to define a type in the parameters
fn secondfunparam(x: i32) {
    // x = x + 1;
    println!("Hello, from the second function with parameters: {}", x);
}

fn thirdfunction(x: i32, y: i32) {
    let z = x + y;
    println!("Hello, from the third function with parameters: {}", z);
}

// testminer1

//generate    prompt miner nepal is in asia  nepal is in a 0.1
// testminer2 prompt miner nepal is in as  0.2
//
// validator1    nepal is in asia ,nepal is in asia
// validator1

fn practise_stringliteral() {
    let mut my_name: &str = "testminer1";

    let mero_name: String = String::from("testminer2");

    println!("{}", mero_name);

    print!("{}", my_name);
    my_name = "testminer2";

    println!("{}", my_name);

    // what we need to understand here is
    // if we declare some string variable without specifying the variable type then it will be the default string type which is &str

    // if we declare String variable then we need to change it to the String type like -> String::from("testminer2")
    // the &str wont work for the dynamic type string.
    let class_name = "hari";

    println!("{}", class_name);

    let class_name = String::from("hari krishna");
    println!("{}", class_name);
}

fn tuple_practice() {
    println!("Hello, from the tuple practice function!");
    let emp_info: (&str, u8) = ("Hari", 24);
    println!(
        "the employe name is {}, and the age is {}",
        emp_info.0, emp_info.1
    );
    let (empoloyeename, employeeage) = emp_info;

    println!(
        "the employe name is {}, and the age is {}",
        empoloyeename, employeeage
    );
}
// function name (variable:datatype)->returntype{return variable;}
fn withreturntype(age: u8) -> u8 {
    return age;
}
