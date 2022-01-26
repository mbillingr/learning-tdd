use crate::stocks::Portfolio;
use crate::stocks::{Bank, Money};
use lazy_static::lazy_static;

lazy_static! {
    static ref BANK: Bank = {
        let mut bank = Bank::new();
        bank.add_exchange_rate("EUR", "USD", 1.2);
        bank.add_exchange_rate("USD", "KRW", 1100.0);
        bank
    };
}

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
    let five_dollars = Money::new(5, "USD");
    let ten_dollars = Money::new(10, "USD");
    let fifteen_dollars = Money::new(15, "USD");

    let mut portfolio = Portfolio::new();
    portfolio = portfolio.add(five_dollars);
    portfolio = portfolio.add(ten_dollars);
    let portfolio_in_dollars = portfolio.evaluate(&BANK, "USD");

    assert_eq!(portfolio_in_dollars, Ok(fifteen_dollars));
}

#[test]
fn test_addition_of_dollars_and_euros() {
    let five_dollars = Money::new(5, "USD");
    let ten_euros = Money::new(10, "EUR");

    let mut portfolio = Portfolio::new();
    portfolio = portfolio.add(five_dollars);
    portfolio = portfolio.add(ten_euros);

    let expected_value = Money::new(17, "USD");
    let actual_value = portfolio.evaluate(&BANK, "USD");
    assert_eq!(actual_value, Ok(expected_value));
}

#[test]
fn test_addition_of_dollars_and_wons() {
    let one_dollar = Money::new(1, "USD");
    let eleven_hundred_won = Money::new(1100, "KRW");

    let mut portfolio = Portfolio::new();
    portfolio = portfolio.add(one_dollar);
    portfolio = portfolio.add(eleven_hundred_won);

    let expected_value = Money::new(2200, "KRW");
    let actual_value = portfolio.evaluate(&BANK, "KRW");
    assert_eq!(actual_value, Ok(expected_value));
}

#[test]
fn test_addition_with_missing_exchange_rates() {
    let one_dollar = Money::new(1, "USD");
    let one_euro = Money::new(1, "EUR");
    let one_won = Money::new(1, "KRW");

    let mut portfolio = Portfolio::new();
    portfolio = portfolio.add(one_dollar);
    portfolio = portfolio.add(one_euro);
    portfolio = portfolio.add(one_won);

    let expected_error = Err(String::from(
        "Missing exchange rate(s): [\"USD->Kalganid\", \"EUR->Kalganid\", \"KRW->Kalganid\"]",
    ));
    let actual_value = portfolio.evaluate(&BANK, "Kalganid");
    assert_eq!(actual_value, expected_error);
}

#[test]
fn test_conversion() {
    let mut bank = Bank::new();
    bank.add_exchange_rate("EUR", "USD", 1.2);
    let ten_euros = Money::new(10, "EUR");
    let actual_converted_money = bank.convert(ten_euros, "USD");
    assert_eq!(actual_converted_money, Ok(Money::new(12, "USD")))
}

#[test]
fn test_conversion_with_missing_exchange_rates() {
    let bank = Bank::new();
    let ten_euros = Money::new(10, "EUR");
    let actual_converted_money = bank.convert(ten_euros, "Kalganid");
    assert_eq!(actual_converted_money, Err("EUR->Kalganid".to_string()))
}
