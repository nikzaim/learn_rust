//Functions
// a function / variable should be written in snake_case

// <--------------------------------------> //
fn main() {
    hello_world();
    tell_height(170);
    human_id("Zaim", 21, 1.7);
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("{}",x);
    println!("{}",add(1, 2));
    
    // calling bmi func
    println!("My Bmi is {:.2}",calculate_bmi(50.0, 1.70));
}

// Hoisting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, Rust!");
}

// you can insert input values
fn tell_height(height: i32){
    println!("My height is {} cm", height)
}

//you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} m.", name, age, height);
}

// functions returning values
fn add(a: i32, b: i32) -> i32{
    a + b
}

// Expressions and Statements
// Expressions : anything that returns a value
// 10 + 5
// true & false
// add(3,4)

// Statement : anything that does not returns a value
//almost all statements in rust end with ';'
// 1. variable declarations: let y = 1;
// 2. function definitions: fn foo() {};
// 3. control flow statements: if condition, else, while condition

// Final example
// BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight: f32, height: f32) -> f32{
    weight / (height * height)
}