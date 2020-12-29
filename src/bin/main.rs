use fer::{Currency, Symbol};

fn main() {
    let b1 = Currency::new_from_str("").unwrap();
    println!("{}", b1.convert_into(Symbol::Jpy, 100_f64));
}
