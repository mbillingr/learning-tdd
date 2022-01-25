import functools
import operator
import unittest


class Portfolio:
    def __init__(self):
        self.moneys = []

    def add(self, *moneys):
        self.moneys.extend(moneys)

    def evaluate(self, currency):
        total = functools.reduce(operator.add, map(lambda m: m.amount, self.moneys), 0)
        return Money(total, currency)


class Money:
    def __init__(self, amount, currency):
        self.amount = amount
        self.currency = currency

    def times(self, multiplier):
        return Money(self.amount * multiplier, self.currency)

    def divide(self, divisor):
        return Money(self.amount / divisor, self.currency)

    def __eq__(self, other):
        return self.amount == other.amount and self.currency == other.currency


class TestMoney(unittest.TestCase):
    def testMultiplication(self):
        five_do = Money(5, "USD")
        ten_ = Money(10, "USD")
        self.assertEqual(ten_, five_do.times(2))

    def testMultiplicationInEuros(self):
        ten_euros = Money(10, "EUR")
        twenty_euros = Money(20, "EUR")
        self.assertEqual(twenty_euros, ten_euros.times(2))

    def testDivision(self):
        original_money = Money(4002, "KRW")
        actual_money_after_division = original_money.divide(4)
        expected_money_after_division = Money(1000.5, "KRW")
        self.assertEqual(expected_money_after_division, actual_money_after_division)

    def testAddition(self):
        five_dollars = Money(5, "USD")
        ten_dollars = Money(10, "USD")
        fifteen_dollars = Money(15, "USD")
        portfolio = Portfolio()
        portfolio.add(five_dollars, ten_dollars)
        self.assertEqual(fifteen_dollars, portfolio.evaluate("USD"))


if __name__ == "__main__":
    unittest.main()
