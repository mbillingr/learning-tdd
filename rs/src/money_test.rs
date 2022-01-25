use crate::stocks::Money;
use crate::stocks::Portfolio;

#[test]
fn test_multiplication() {
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
