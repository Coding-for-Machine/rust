
fn main() {
    print!("Salom, Rust\n"); // Salom, Rust
    println!("Salom, Rust"); // Salom, Rust
    
    println!("{}, {}", "Salom", "Rust"); // format
    print!("{}, {}\n", "Salom", "Rust");

    println!("{0}, {1}", "Salom", "Rust");
    
    println!("{salom}, {rust}", salom="Salom", rust="Rust");
    println!("{:?}", [1,2,3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]);
    /*
      [
        1,
        2,
        3
      ]
    */
    println!(); // new line
    let (salom, rust) = ("Salom", "Rust");
    println!("{salom}, {rust}");
    let x = format!("{}, {}", "Salom", "Rust" );
    println!("{}", x);
    
}
