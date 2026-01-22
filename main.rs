use std::io;

fn main() {
    let mut a: String = String::from("");
    println!("Write a");
    io::stdin()
        .read_line(&mut a)
        .expect("Not a valid i32 number!");
    let mut b: String = String::from("");
    println!("Write b");
    io::stdin()
        .read_line(&mut b)
        .expect("Not a valid i32 number!");
    let mut c: String = String::from("");
    println!("Write c");
    io::stdin()
        .read_line(&mut c)
        .expect("Not a valid i32 number!");
    
    let a1: f32 = match a.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number, not '{}'", a.trim());
            return;
        }
    };
    let b1: f32 = match b.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number, not '{}'", b.trim());
            return;
        }
    };
    let c1: f32 = match c.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number, not '{}'", c.trim());
            return;
        }
    };

    println!("NOTE: Negative numbers in showcase of quadratic equation will be displayed as +-");
    println!("{a1}x²+{b1}x+{c1}=0");
    println!("a = {a1}, b = {b1}, c = {c1}");

    let d: f32 = b1.powf(2.0) - 4.0 * a1 * c1;
    println!("D = {d}");

    let a2: f32 = 2.0 * a1;

    let x1: f32 = -b1 + d.sqrt();
    let x1: f32 = x1 / a2;
    println!("x1 = {x1}");

    let x2: f32 = -b1 - d.sqrt();
    let x2: f32 = x2 / a2;
    println!("x2 = {x2}");
}
