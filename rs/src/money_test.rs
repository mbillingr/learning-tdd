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
        let total: f64 = self.holdings.iter().map(|m| m.amount).sum();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication_in_dollars() {
        let five_dollars = Money::new(5, "USD");
        let ten_dollars = Money::new(10, "USD");
        assert_eq!(five_dollars.times(2), ten_dollars)
    }

    #[test]
    fn test_multiplication_in_euros() {
        let ten_euros = Money::new(10, "EUR");
        let twenty_euros = Money::new(20, "EUR");
        assert_eq!(ten_euros.times(2), twenty_euros);
    }

    #[test]
    fn test_devision() {
        let original_money = Money::new(4002, "KRW");
        let expected_result = Money::new(1000.5, "KRW");
        let actual_result = original_money.divide(4);
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_addition() {
        let mut portfolio = Portfolio::new();

        let five_dollars = Money::new(5, "USD");
        let ten_dollars = Money::new(10, "USD");
        let fifteen_dollars = Money::new(15, "USD");

        portfolio = portfolio.add(five_dollars);
        portfolio = portfolio.add(ten_dollars);
        let portfolio_in_dollars = portfolio.evaluate("USD");

        assert_eq!(portfolio_in_dollars, fifteen_dollars);
    }
}
