fn main() {
    let mut x = 5;
    { // Using a block to limit scope 
        let y = &mut x;
        *y = 6;
        println!("x = {}", x);
    }
    { // Creating a new mutable reference in a new scope
        let z = &mut x;
        *z = 7;
        println!("x = {}", x);
    }
    println!("x = {}", x);
}