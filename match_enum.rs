#[allow(dead_code)] // We don't actually use Transparent

enum Color {
    Transparent,
    RGB(u32, u32, u32)
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Transparent   => println!("See-through!"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b)
    }
}
