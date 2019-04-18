fn main() {

    let mut vector = vec![3, 4, 5];
    for val in vector.iter_mut() {
        match val {
            5 => *val *= 2,
            _ => ()
        }
    }
    println!("{:?}", vector);

}
