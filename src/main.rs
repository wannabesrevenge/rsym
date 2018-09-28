mod sym;

fn main() {
     let a = sym::value::get_concrete::<u8>(10);
     let b = sym::value::get_concrete::<u8>(50);
     let c = sym::value::get_symbolic::<u8>("x".to_string());
     let d = sym::value::get_symbolic::<u8>("y".to_string());

     let e = a.clone() + c.clone();
     let f = b.clone() ^ d.clone();
     let g = e.clone() * f.clone();
     println!("{}", g);
}
