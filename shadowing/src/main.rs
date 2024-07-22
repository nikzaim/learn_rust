// Shadowing
// shadowing is not the same as mut
// use let keywords and same variable name to perform shadowing

fn main() {

    // first x
    let x = 5;

    // new x shadowing the first x
    let x = x + 1;

    // shadowing with inner scope
    {
        let x = x * 2;
        println!("Inner scope x is {}", x);
    }
    
    let x = 10;
    println!("Outer scope x is {}", x);
}
