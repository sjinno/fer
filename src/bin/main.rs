use fer::{currency::Currency, Symbol};
use std::io;

fn main() -> io::Result<()> {
    let base = Currency::new(Symbol::Usd).unwrap();
    println!(
        "100 USD => {:.2} JPY",
        base.convert_into(Symbol::Jpy, 100 as f64)
    );
    println!(
        "1000 JPY => {:.2} USD",
        base.convert_from(Symbol::Jpy, 1000 as f64)
    );
    assert_eq!(base.convert_from(Symbol::Usd, 1.0), 1.0);
    Ok(())
}
