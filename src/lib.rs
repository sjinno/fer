use serde::Deserialize;
use std::collections::HashMap;

/// Main type for storing retrieved data from a third-party API.
#[derive(Deserialize, Debug)]
struct Currency {
    rates: HashMap<String, f64>,
    base: String,
    date: String,
}

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

fn get_symbol(base_currency: Symbol) -> &'static str {
    match base_currency {
        Symbol::Cad => "CAD",
        Symbol::Hkd => "HKD",
        Symbol::Isk => "ISK",
        Symbol::Php => "PHP",
        Symbol::Dkk => "DKK",
        Symbol::Huf => "HUF",
        Symbol::Czk => "CZK",
        Symbol::Gbp => "GBP",
        Symbol::Ron => "RON",
        Symbol::Sek => "SEK",
        Symbol::Idr => "IDR",
        Symbol::Brl => "BRL",
        Symbol::Rub => "RUB",
        Symbol::Hrk => "HRK",
        Symbol::Jpy => "JPY",
        Symbol::Thb => "THB",
        Symbol::Chf => "CHF",
        Symbol::Eur => "EUR",
        Symbol::Myr => "MYR",
        Symbol::Bgn => "BGN",
        Symbol::Try => "TRY",
        Symbol::Cny => "CNY",
        Symbol::Nok => "NOK",
        Symbol::Nzd => "NZD",
        Symbol::Zar => "ZAR",
        Symbol::Usd => "USD",
        Symbol::Sgd => "SGD",
        Symbol::Aud => "AUD",
        Symbol::Ils => "ILS",
        Symbol::Krw => "KRW",
        Symbol::Pln => "PLN",
    }
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
    ///     let base = Currency::new(Symbol::Usd);
    ///     assert_eq!("USD".to_string(), base.base);
    ///     assert_eq!(base.convert_from(Usd, 1), 1.0);
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

    pub fn convert_into(&self, currency: Symbol, amount: f64) -> f64 {
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        currency_rate * amount
    }

    pub fn convert_from(&self, currency: Symbol, amount: f64) -> f64 {
        let base_rate = self.rates.get(&self.base).unwrap();
        let currency_rate = self.rates.get(get_symbol(currency)).unwrap();
        base_rate / currency_rate * amount
    }
}
