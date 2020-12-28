use fer::{currency::Currency, Symbol};
use std::io;

fn main() -> io::Result<()> {
    let base = Currency::new(Symbol::Gbp).unwrap();
    let usd = Currency::new(Symbol::Gbp).unwrap();

    // println!(
    //     "100 USD => {:.2} JPY",
    //     base.convert_into(Symbol::Jpy, 100 as f64)
    // );
    // println!(
    //     "1000 JPY => {:.2} USD",
    //     base.convert_from(Symbol::Jpy, 1000 as f64)
    // );
    assert_eq!(usd.convert_from(Symbol::Usd, 1.0), 1.0);
    // let base = Currency::new(Symbol::Usd).unwrap();

    println!(
        "1000 USD in JPY: {:.2}",
        base.convert_into(Symbol::Jpy, 1000_f64)
    );
    println!(
        "1_000_000 USD in JPY: {:.2}",
        base.convert_into(Symbol::Usd, 1_000_000_f64)
    );

    // println!(
    //     "1000 JPY ======> {} GBP",
    //     base.convert_from(Symbol::Jpy, 1000_f64)
    // );
    // println!(
    //     "1_000_000 USD ======> {} GBP",
    //     base.convert_from(default, 1_000_000_f64)
    // );

    Ok(())
}
