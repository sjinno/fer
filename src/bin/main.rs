use fer::{Currency, Symbol};

fn main() {
    // let b1 = Currency::new_from_str("usd").unwrap();
    // println!("{}", b1.convert_into_from_str("jpy", 100_f64));
    let usd = Currency::new(Symbol::Usd).unwrap();
    let eur = Currency::new_from_str("eUR").unwrap();
    println!("{:.0}", usd.convert_into_from_str("jpy", 100_f64));
    println!("{:.0}", eur.convert_into(Symbol::Jpy, 100_f64));
}
