use std::io;
const GLOBAL_VARIABLE: &str = "saga";

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
    print_globalariable();

    ownership_example();

    clone_practise();

    borrowing_practise();

    refrence();
    array_practise();

    passing_array();

    vector_practise();
    if_else_practise();
    loop_practise();
    while_practise();
    for_practise();

    match_keyword();

    input_output();
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

// lets dive into the memory management of the rust.
// the static variables are stored in the stack and dynamic variable are stored in the heap.

fn print_globalariable() {
    println!("global variable {}", GLOBAL_VARIABLE)
}

fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // this will give an error
    println!("{}", s2);
    // this will give an error because the ownership of the s1 is moved to s2
    // println!("{}", s1);
    // as we  know that the String type a dynamic variable allocates the memeory in the heap soo if the memory is allocated for the given variable then the pointer is pointing to the allocated memory of that variable and s1 is the owner of that value (hello world !).soo when we try to access the variable s1 after the ownership is moved to s2 then it will give an error because the ownership of the value is moved to s2.
    // the ownership of the value is moved to the s2 soo the value of the s1 is not valid anymore.

    // but in case of int, &str ,float there wont be the rule of the ownership.
}

// clone in rust

fn clone_practise() {
    let s1 = String::from("hello from the clone project ");
    // this will create a copy of the s1 and reflect it on s2 but this approach uses the memeory inefficiently

    let s2 = s1.clone();
    println!("{}", s2);
}

fn borrowing_practise() {
    // also called mutable reference

    // to overcome the copy of the ownership of the variables we are going to use the refrence operator to borrow the ownership for the certain time

    let mut s1 = String::from("hello from the borrowing project ");

    // here we are borrowing ownership from the s1 to s2 using refrence operator
    let s2 = &mut s1;
    s2.push_str("hello world");

    println!("{}", s2);

    // here we are writing operation at the same time by borrowing

    // yaha samma s1 ko sapati s2 ley ligyako xa
    // read operation is possible at the same time or write and read is possible
    let s3 = &mut s1;
    // aba s3 ko sapati s3 ley ligyo

    s3.push_str("hello nepal");

    // s2 ta ailya empty xa because s3 ley ownership ligyako xa

    println!("{} ", s3);
}

fn refrence() {
    let x = 30;

    println!("the address of x is {:p}", &x);

    // &x is the refrence of the x value address
    let y = &x;

    // here we are printing the address value of the x value address
    println!("the address of y is {:p}", y);
    // the actual address of the y
    println!("the address of y is now {:p}", &y);
}

fn array_practise() {
    let mut array1;
    array1 = [1, 2, 3, 4, 5];

    println!("{}", array1[0]);

    array1[3] = 10;

    println!("{:?}", array1);

    println!("{:?}", array1.len());
}

fn passing_array() {
    let mut array1: [&str; 5] = ["hello", "world", "from", "the", "array"];

    write_array(array1);

    mutable_array(&mut array1);

    println!("array1 after completing modification{:?}", array1);
}

fn write_array(mut array11: [&str; 5]) {
    array11[0] = "sagar";
    println!("the array after modification{:?}", array11);
}

fn mutable_array(arr4: &mut [&str; 5]) {
    arr4[0] = "sagar";
    println!("the array after modification{:?}", arr4);
}

fn vector_practise() {
    // declaration of the vector dynamic array

    // let mut v: Vec<i32> = Vec::new();

    // initialization of the vector dynamic array

    let mut v = vec![1, 2, 3, 4, 5];
    let mut vector_name: Vec<&str> = Vec::new();
    vector_name.push("sagar");
    vector_name.push("hari");
    vector_name.push("krishna");
    println!("{:?}", vector_name);

    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);
}

fn if_else_practise() {
    let x = 5;
    // remember there is no paranthese in the if else statement
    if x > 5 {
        println!("x is greater than 5");
    } else if x == 5 {
        println!("x is equal to 5");
    } else {
        println!("x is less than 5");
    }
}

fn loop_practise() {
    let mut i = 0;
    loop {
        println!("the value of i is: {}", i);
        i = i + 1;
        if i == 5 {
            break;
        }
    }
}

fn while_practise() {
    let mut i = 0;
    while i < 5 {
        println!("the value of i is: {}", i);
        i = i + 1;
    }
}

fn for_practise() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the value of element is: {}", element);
    }
}

fn match_keyword() {
    let x = 10;
    match x {
        1 | 2 => println!("one"),
        7 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        // default value
        _ => println!("other"),
    }
}

fn input_output() {
    // taking input from the user
    let mut input = String::new();
    println!("Enter your name: ");

    // mutable soo we are making refrence to the mutable variable
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Your name is: {}", input);
}
