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

    pub fn evaluate(&self, bank: &Bank, currency: &'static str) -> Result<Money, String> {
        let (total, err) =
            self.holdings
                .iter()
                .fold((0.0, vec![]), |(total, mut err), m| {
                    match bank.convert(*m, currency) {
                        Ok(Money { amount, .. }) => (total + amount, err),
                        Err(conversion) => {
                            err.push(conversion);
                            (total, err)
                        }
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

pub struct Bank {
    exchange_rates: HashMap<(&'static str, &'static str), f64>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            exchange_rates: HashMap::new(),
        }
    }

    pub fn add_exchange_rate(&mut self, from: &'static str, to: &'static str, rate: f64) {
        self.exchange_rates.insert((from, to), rate);
    }

    pub fn convert(&self, money: Money, currency: &'static str) -> Result<Money, String> {
        if money.currency == currency {
            return Ok(money);
        }

        match self.exchange_rates.get(&(money.currency, currency)) {
            Some(rate) => Ok(Money::new(money.amount * rate, currency)),
            None => Err(format!("{}->{}", money.currency, currency)),
        }
    }
}
