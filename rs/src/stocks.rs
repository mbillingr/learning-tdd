use std::collections::HashMap;

pub struct Portfolio {
    holdings: Vec<Money>,
}

impl Portfolio {
    pub fn new() -> Self {
        Portfolio { holdings: vec![] }
    }

    pub fn add(&self, money: Money) -> Self {
        let mut holdings = self.holdings.clone();
        holdings.push(money);
        Portfolio { holdings }
    }

    pub fn evaluate(&self, currency: &'static str) -> Result<Money, String> {
        let (total, err) = self
            .holdings
            .iter()
            .fold((0.0, vec![]), |(total, mut err), m| {
                if let Some(amount) = convert(m, currency) {
                    (total + amount, err)
                } else {
                    err.push(format!("{}->{}", m.currency, currency));
                    (total, err)
                }
            });

        if err.is_empty() {
            Ok(Money::new(total, currency))
        } else {
            Err(format!("Missing exchange rate(s): {:?}", err))
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Money {
    amount: f64,
    currency: &'static str,
}

impl Money {
    pub fn new(amount: impl Into<f64>, currency: &'static str) -> Self {
        Money {
            amount: amount.into(),
            currency,
        }
    }

    pub fn times(&self, multiplier: u64) -> Self {
        Money::new(self.amount * multiplier as f64, self.currency)
    }

    pub fn divide(&self, divisor: u64) -> Self {
        Money::new(self.amount / divisor as f64, self.currency)
    }
}

fn convert(money: &Money, currency: &'static str) -> Option<f64> {
    if money.currency == currency {
        Some(money.amount)
    } else {
        let mut exchange_rates = HashMap::new();
        exchange_rates.insert(("EUR", "USD"), 1.2);
        exchange_rates.insert(("USD", "KRW"), 1100.0);
        exchange_rates
            .get(&(money.currency, currency))
            .map(|rate| money.amount * rate)
    }
}
