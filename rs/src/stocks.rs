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

    pub fn evaluate(&self, currency: &'static str) -> Money {
        let total: f64 = self.holdings.iter().map(|m| convert(m, currency)).sum();
        Money::new(total, currency)
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

fn convert(money: &Money, currency: &'static str) -> f64 {
    if money.currency == currency {
        money.amount
    } else {
        let eur_to_usd = 1.2;
        money.amount * eur_to_usd
    }
}