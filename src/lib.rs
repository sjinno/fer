use serde::Deserialize;
use std::collections::HashMap;

/// Main type for storing retrieved data from a third-party API.
#[derive(Deserialize, Debug)]
pub struct Currency {
    rates: HashMap<String, f64>,
    pub base: String,
    date: String,
}

/// List of three-letter symbols of major currencies.
#[derive(Copy, Clone)]
pub enum Symbol {
    Cad, // Canadian Dollar
    Hkd, // Hong Kong Dollar
    Isk, // Icelandic Krona
    Php, // Philippine Peso
    Dkk, // Danish Krone
    Huf, // Hungarian Forint
    Czk, // Czec Kouruna
    Gbp, // Great British Pound
    Ron, // Romanian New Leu
    Sek, // Swedish Krona
    Idr, // Indonesian Rupiah
    Brl, // Brazillian Real
    Rub, // Russian Ruble
    Hrk, // Croatian Kuna
    Jpy, // Japanese Yen
    Thb, // Thai Baht
    Chf, // Swiss Franc
    Eur, // Euro
    Myr, // Malaysian Ringgit
    Bgn, // Bulgaria Lev
    Try, // Turkish Lira
    Cny, // Chinese Yuan
    Nok, // Norwesian Krone
    Nzd, // New Zealand Dollar
    Zar, // South African Rand
    Usd, // US Dollar
    Sgd, // Singapore Dollar
    Aud, // Australian Dollar
    Ils, // Israeli Shekel
    Krw, // South Korean Won
    Pln, // Polish Zloty
}

/// Convert str into Symbol type.
#[allow(dead_code)]
pub fn match_symbol(currency: &str) -> Symbol {
    let curr = currency.to_uppercase();
    match curr.as_str() {
        "CAD" => Symbol::Cad,
        "HKD" => Symbol::Hkd,
        "ISK" => Symbol::Isk,
        "PHP" => Symbol::Php,
        "DKK" => Symbol::Dkk,
        "HUF" => Symbol::Huf,
        "CZK" => Symbol::Czk,
        "GBP" => Symbol::Gbp,
        "RON" => Symbol::Ron,
        "SEK" => Symbol::Sek,
        "IDR" => Symbol::Idr,
        "BRL" => Symbol::Brl,
        "RUB" => Symbol::Rub,
        "HRK" => Symbol::Hrk,
        "JPY" => Symbol::Jpy,
        "THB" => Symbol::Thb,
        "CHF" => Symbol::Chf,
        "EUR" => Symbol::Eur,
        "MYR" => Symbol::Myr,
        "BGN" => Symbol::Bgn,
        "TRY" => Symbol::Try,
        "CNY" => Symbol::Cny,
        "NOK" => Symbol::Nok,
        "NZD" => Symbol::Nzd,
        "ZAR" => Symbol::Zar,
        "USD" => Symbol::Usd,
        "SGD" => Symbol::Sgd,
        "AUD" => Symbol::Aud,
        "ILS" => Symbol::Ils,
        "KRW" => Symbol::Krw,
        "PLN" => Symbol::Pln,
        _ => Symbol::Eur, // Default
    }
}

fn get_symbol(currency: Symbol) -> &'static str {
    match currency {
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

pub fn list_symbols() -> &'static str {
    r#"
```
CAD  Canadian Dollar
HKD  Hong Kong Dollar
ISK  Icelandic Krona
PHP  Philippine Peso
DKK  Danish Krone
HUF  Hungarian Forint
CZK  Czec Kouruna
GBP  Great British Pound
RON  Romanian New Leu
SEK  Swedish Krona
IDR  Indonesian Rupiah
BRL  Brazillian Real
RUB  Russian Ruble
HRK  Croatian Kuna
JPY  Japanese Yen
THB  Thai Baht
CHF  Swiss Franc
EUR  Euro
MYR  Malaysian Ringgit
BGN  Bulgaria Lev
TRY  Turkish Lira
CNY  Chinese Yuan
NOK  Norwesian Krone
NZD  New Zealand Dollar
ZAR  South African Rand
USD  US Dollar
SGD  Singapore Dollar
AUD  Australian Dollar
ILS  Israeli Shekel
KRW  South Korean Won
PLN  Polish Zloty
```"#
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

#[cfg(test)]
mod tests {
    use super::{match_symbol, Currency, Symbol};
    #[test]
    fn test_lib() {
        let usd = match_symbol("usd");
        let base = Currency::new(usd).unwrap();
        assert_eq!("USD".to_string(), base.base);
        assert_eq!(base.convert_into(Symbol::Usd, 1.0), 1.0);
        assert_eq!(base.convert_from(Symbol::Usd, 1_000_000_f64), 1_000_000_f64);
    }
}
