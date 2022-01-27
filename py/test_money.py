import unittest

from bank import Bank
from money import Money
from portfolio import Portfolio


class TestMoney(unittest.TestCase):
    def setUp(self):
        self.bank = Bank()
        self.bank.add_exchange_rate("EUR", "USD", 1.2)
        self.bank.add_exchange_rate("USD", "KRW", 1100)

    def testMultiplication(self):
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
        self.assertEqual(fifteen_dollars, portfolio.evaluate(self.bank, "USD"))

    def testAdditionOfDollarsAndEuros(self):
        five_dollars = Money(5, "USD")
        ten_euros = Money(10, "EUR")
        portfolio = Portfolio()
        portfolio.add(five_dollars, ten_euros)
        expected_value = Money(17, "USD")
        self.assertEqual(expected_value, portfolio.evaluate(self.bank, "USD"))

    def testAdditionOfDollarsAndWons(self):
        one_dollars = Money(1, "USD")
        eleven_hundred_wons = Money(1100, "KRW")
        portfolio = Portfolio()
        portfolio.add(one_dollars, eleven_hundred_wons)
        expected_value = Money(2200, "KRW")
        self.assertEqual(expected_value, portfolio.evaluate(self.bank, "KRW"))

    def testAdditionWithMultipleMissingExchangeRates(self):
        one_dollars = Money(1, "USD")
        one_euro = Money(1, "EUR")
        one_won = Money(1, "KRW")
        portfolio = Portfolio()
        portfolio.add(one_dollars, one_euro, one_won)
        with self.assertRaisesRegex(
            Exception,
            "Missing exchange rate\(s\): \[USD\->Kalganid,EUR->Kalganid,KRW->Kalganid]",
        ):
            portfolio.evaluate(self.bank, "Kalganid")

    def testConversion(self):
        ten_euros = Money(10, "EUR")
        self.assertEqual(self.bank.convert(ten_euros, "USD"), Money(12, "USD"))

        self.bank.add_exchange_rate("EUR", "USD", 1.3)
        self.assertEqual(self.bank.convert(ten_euros, "USD"), Money(13, "USD"))

    def testConversionWithMissingExchangeRate(self):
        bank = Bank()
        ten_euros = Money(10, "EUR")
        with self.assertRaisesRegex(Exception, "EUR->Kalganid"):
            bank.convert(ten_euros, "Kalganid")


if __name__ == "__main__":
    unittest.main()
