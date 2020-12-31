use fer::{list_symbols, match_symbol, Currency};

fn main() {
    // let b1 = Currency::new_from_str("usd").unwrap();
    // println!("{}", b1.convert_into_from_str("jpy", 100_f64));
    let usd = match_symbol("usd");
    let jpy = match_symbol("jpy");
    let eur = match_symbol("");
    let us = Currency::new(usd).unwrap();
    let eu = Currency::new(eur).unwrap();
    println!("{:.0}", us.convert_into(jpy, 100_f64));
    println!("{:.0}", eu.convert_into(jpy, 100_f64));
    println!("{}", list_symbols());
}
