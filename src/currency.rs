use crate::symbols::{get_symbol, Symbol};
use serde::Deserialize;
use std::collections::HashMap;

/// Main type for storing retrieved data from a third-party API.
#[derive(Deserialize, Debug)]
pub struct Currency {
    rates: HashMap<String, f64>,
    base: String,
    date: String,
}

impl Currency {
    /// Create a new `Currency` instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fer::{Currency, Symbol};
    ///
    /// fn main() {
    ///     let base = Currency::new(Symbol::Usd).unwrap();
    ///     assert_eq!(base.convert_from(Symbol::Usd, 1_f64), 1.0);
    /// }
    /// ```
    #[tokio::main]
    pub async fn new(base_currency: Symbol) -> Result<Self, Box<dyn std::error::Error>> {
        let base_currency: &str = get_symbol(base_currency);
        let url: &str = "https://api.exchangeratesapi.io/latest";
        let mut base_url = url.to_owned();
        base_url.push_str(&format!("?base={}", base_currency));
        let resp = reqwest::get(&base_url).await?.json::<Self>().await?;
        Ok(resp)
    }

    /// Convert the amount of base currency into your preferrd currency.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fer::{Currency, Symbol};
    ///
    /// fn main() {
    ///     let base = Currency::new(Symbol::Usd).unwrap();
    ///     println!("1000 USD in JPY: {}", base.convert_into(Symbol::Jpy, 1000_f64));
    ///     println!("1_000_000 USD in JPY: {}", base.convert_into(Symbol::Jpy, 1_000_000_f64));
    /// }
    /// ```
    pub fn convert_into(&self, currency: Symbol, amount: f64) -> f64 {
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        currency_rate * amount
    }

    /// Similarly, this will convert the amount of your preferred currency into base currency.
    ///
    /// # Example
    ///
    /// ```rust
    /// use fer::{Currency, Symbol};
    ///
    /// fn main() {
    ///     let base = Currency::new(Symbol::Usd).unwrap();
    ///     println!("1000 JPY in USD: {}", base.convert_from(Symbol::Jpy, 1000_f64));
    ///     println!("1_000_000 JPY in USD: {}", base.convert_from(Symbol::Jpy, 1_000_000_f64));
    /// }
    /// ```
    pub fn convert_from(&self, currency: Symbol, amount: f64) -> f64 {
        let base_rate = self.rates.get(&self.base).unwrap();
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        base_rate / currency_rate * amount
    }
}
