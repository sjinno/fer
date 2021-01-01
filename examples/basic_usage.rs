use fer::{list_symbols, match_symbol, Currency, Symbol};

fn main() {
    let usd = Symbol::Usd;
    let jpy = Symbol::Jpy;
    let base = Currency::new(usd).unwrap();
    let amount = 1.0;
    let res = base.convert_into(jpy, amount);
    println!("{}", res);
}
