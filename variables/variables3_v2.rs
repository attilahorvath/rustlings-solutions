// Make me compile! Scroll down for hints :)

fn main() {
    let x = 3;
    println!("Number {}", x);
    let x = 5; // Alternatively, we could shadow the previous variable binding
    println!("Number {}", x);
}































// In Rust, variable bindings are immutable by default. But here we're trying
// to reassign a different value to x! There's a keyword we can use to make
// a variable binding mutable instead.
