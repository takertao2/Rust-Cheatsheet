fn main() {

    struct Foo { x: (u32, u32), y: u32 }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {} ", a, b, y);

    // You can ignore some variables
    let Foo { y, .. } = foo;
    println!("y = {}", y);

}
