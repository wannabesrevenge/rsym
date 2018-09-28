mod sym;
mod memory;

// use self::sym::value::IsValue;


fn main() {
     let x = sym::value::SymbolicVariable::<u8>::new("x".to_string());
     let y = sym::value::SymbolicVariable::<u8>::new("y".to_string());
     let z = x + y;
     println!("{}", z);
    // x.print();
}
