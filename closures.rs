fn main() {
    // Closures can save us a lot of code (through inference)
    fn add_five_function (i: i32) -> i32 {
        i + 5
    }
    let add_five_closure = |i| i + 5;
    let i = 1;
    println!("Adding 1...");
    println!("Via function: {}", add_five_function(i));
    println!("Via closure: {}", add_five_closure(i));

    // Closures can take no arguments
    let five = || 5;
    println!("A closure returning five: {}", five());

}
