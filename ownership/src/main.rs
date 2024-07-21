// Ownership, Borrowing and References

// C, C++ -> memory management control issue
// garbage collector solved this issue, but created a new issue -> Slow Performance:
// [ stopping and resuming the program ]

// Ownership Rules
// 1- Each value in Rust has a variable that's its owner
// 2- There can only be one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped.

fn main() {
    //Example: Each value in Rust has a variable that's its owner
    // RUST value is owned by the s1(owner)
    let s1 = String::from("RUST");
    let len = calcualte_length(&s1);
    println!("Length of '{}' is {}.", s1, len);

    // Example: There can only be one owner at a time.
    let i = String::from("RUST");
    // ownership transfer to variable j
    let j = i;
    // println!("{}", i); -> error because i no longer the owner
    println!("{}", j);

    // Example: When the owner goes out of scope, the value will be dropped.
    let x = String::from("RUST");
    let size = calcualte_length(&x);
    println!("Length of '{}' is {}.", x, size);

    // <--------------------------------------> //

    // Borrowing and References
    // Safety and Performance
    // Borrowing and references are powerful concepts

    // Understanding References
    // References: Enable you to borrow values without taking ownership
    // Immutable Reference
    // Mutable Reference
    // Create Reference by add "&"

    // -I- Immutable Reference
    let a: i32 = 5;
    // b referring(borrowing) to(from) a(owner)
    let b: &i32 = &a;
    println!("Value of a: {}.", a);
    println!("Value of b: {}.", b);

    // -M- Mutable Reference
    let mut c: i32 = 5;
    // d referring(borrowing) to(from) c(owner)
    let d: &mut i32 = &mut c;
    // '*' is like dereferencing a reference '&'
    *d += 1;
    println!("Value of c: {}.", c);

    // Demonstration on one mutable reference or many immutable references(relate with struct and impl at outer scope)
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();

    // Immutable borrow to withdraw money
    account.withdraw(45.5);
    account.check_balance();

}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // mutable
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // immutable
    fn check_balance(&self){
        println!("Account owned by {} has a balance of {:.2}", self.owner, self.balance)
    }
}

// x goes out of scope and its value will be dropped
// cannot find value `x` in this scope
// fn printDropped(s: &String){
//     println!("{}", &x)
// }

fn calcualte_length(s: &String) -> usize{
    s.len()
}


