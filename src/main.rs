use serde::Deserialize;
use std::collections::HashMap;
use std::io;

#[derive(Deserialize, Debug)]
struct Currency {
    rates: HashMap<String, f64>,
    base: String,
    date: String,
}

impl Currency {
    #[tokio::main]
    async fn new(base_currency: &str) -> Result<Currency, Box<dyn std::error::Error>> {
        const URL: &str = "https://api.exchangeratesapi.io/latest";
        let mut base_url = URL.to_owned();
        base_url.push_str(&format!("?base={}", base_currency.to_uppercase()));
        println!("{:#?}", &base_url);
        let resp = reqwest::get(&base_url).await?.json::<Currency>().await?;
        println!("{:#?}", resp);
        Ok(resp)
    }

    fn convert_into(&self, preferred_currency: &str, amount: f64) -> f64 {
        let preferred_currency_rate = self.rates.get(&preferred_currency.to_uppercase()).unwrap();

        preferred_currency_rate * amount
    }

    fn convert_from(&self, preferred_currency: &str, amount: f64) -> f64 {
        let base_rate = self.rates.get(&self.base).unwrap();
        let preferred_currency_rate = self.rates.get(&preferred_currency.to_uppercase()).unwrap();

        base_rate / preferred_currency_rate * amount
    }
}

fn main() -> io::Result<()> {
    let base = Currency::new("usd").unwrap();
    println!("100 USD => {:.2} JPY", base.convert_into("JPY", 100 as f64));
    println!(
        "1000 JPY => {:.2} USD",
        base.convert_from("JPY", 1000 as f64)
    );
    Ok(())
}
