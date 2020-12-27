use serde::Deserialize;
use std::collections::HashMap;
use std::io;
use Symbol::*;

/// Main type for storing retrieved data from a third-party API.
#[derive(Deserialize, Debug)]
struct Currency {
    rates: HashMap<String, f64>,
    base: String,
    date: String,
}

#[allow(unused)]
enum Symbol {
    Cad,
    Hkd,
    Isk,
    Php,
    Dkk,
    Huf,
    Czk,
    Gbp,
    Ron,
    Sek,
    Idr,
    Brl,
    Rub,
    Hrk,
    Jpy,
    Thb,
    Chf,
    Eur,
    Myr,
    Bgn,
    Try,
    Cny,
    Nok,
    Nzd,
    Zar,
    Usd,
    Sgd,
    Aud,
    Ils,
    Krw,
    Pln,
}

#[allow(non_camel_case_types)]
fn get_symbol(base_currency: Symbol) -> &'static str {
    // use Symbol::*;
    match base_currency {
        Cad => "CAD",
        Hkd => "HKD",
        Isk => "ISK",
        Php => "PHP",
        Dkk => "DKK",
        Huf => "HUF",
        Czk => "CZK",
        Gbp => "GBP",
        Ron => "RON",
        Sek => "SEK",
        Idr => "IDR",
        Brl => "BRL",
        Rub => "RUB",
        Hrk => "HRK",
        Jpy => "JPY",
        Thb => "THB",
        Chf => "CHF",
        Eur => "EUR",
        Myr => "MYR",
        Bgn => "BGN",
        Try => "TRY",
        Cny => "CNY",
        Nok => "NOK",
        Nzd => "NZD",
        Zar => "ZAR",
        Usd => "USD",
        Sgd => "SGD",
        Aud => "AUD",
        Ils => "ILS",
        Krw => "KRW",
        Pln => "PLN",
    }
}

impl Currency {
    /// Create a new `Currency` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fer::{Currency, Symbol::*};
    ///
    /// fn main() {
    ///     let base = Currency::new("usd");
    ///     
    /// }
    /// ```
    #[tokio::main]
    async fn new(base_currency: Symbol) -> Result<Self, Box<dyn std::error::Error>> {
        let base_currency: &str = get_symbol(base_currency);
        let url: &str = "https://api.exchangeratesapi.io/latest";
        let mut base_url = url.to_owned();
        base_url.push_str(&format!("?base={}", base_currency));
        // println!("{:#?}", &base_url);
        let resp = reqwest::get(&base_url).await?.json::<Self>().await?;
        // println!("{:#?}", resp);
        Ok(resp)
    }

    fn list_currencies(&self) {
        &self
            .rates
            .iter()
            .for_each(|(key, _val)| println!("{}", key));
    }

    fn convert_into(&self, currency: Symbol, amount: f64) -> f64 {
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        currency_rate * amount
    }

    fn convert_from(&self, currency: Symbol, amount: f64) -> f64 {
        let base_rate = self.rates.get(&self.base).unwrap();
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        base_rate / currency_rate * amount
    }
}

fn main() -> io::Result<()> {
    let base = Currency::new(Usd).unwrap();
    println!("100 USD => {:.2} JPY", base.convert_into(Jpy, 100 as f64));
    println!("1000 JPY => {:.2} USD", base.convert_from(Jpy, 1000 as f64));
    assert_eq!(base.convert_from(Usd, 1.0), 1.0);
    base.list_currencies();
    Ok(())
}
