// Compound Data Types
// arrays, tuples, slices, and strings(slice string)

fn main() {

    // Arrays
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    // this would failed bcuz of diff type
    // let mix = [1,2,"three",true];
    // println!("Mix Array: {:?}", mix)

    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // <--------------------------------------> //

    // Tuples
    // let human: (&str, i16, bool)= ("Alice", 30, false);
    let human: (String, i16, bool)= ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    // we can mix primitive and compound types inside tuple
    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // tuple inside tuple
    let mut my_mix_tuple = ("Kratos", 23, true, ("Kratos", 23, true));
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // <--------------------------------------> //

    // Slices: [1,2,3,4,5]
    // Dynamic size
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["lion", "elephant", "crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // <--------------------------------------> //
    // Strings VS String Slices (&str)

    // Strings [ growable, mutable, owned string type ]
    // stored in heap
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah");
    println!("Stone Cold Says: {}", stone_cold);

    // String Slices (&str) 
    let string: String = String::from("Hello, World");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}
