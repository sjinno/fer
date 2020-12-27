pub enum Symbol {
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

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Usd
    }
}

pub fn get_symbol(base_currency: Symbol) -> &'static str {
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
