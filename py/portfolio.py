import functools
import operator

from money import Money


class Portfolio:
    def __init__(self):
        self.moneys = []
        self._eur_to_usd = 1.2

    def add(self, *moneys):
        self.moneys.extend(moneys)

    def evaluate(self, currency):
        total = 0
        failures = []
        for m in self.moneys:
            try:
                total += self.__convert(m, currency)
            except KeyError as ke:
                failures.append(ke)

        if not failures:
            return Money(total, currency)

        failure_message = ",".join(f.args[0] for f in failures)
        raise Exception("Missing exchange rate(s): [" + failure_message + "]")

    def __convert(self, money, currency):
        if money.currency == currency:
            return money.amount
        exchange_rates = {"EUR->USD": 1.2, "USD->KRW": 1100}
        key = money.currency + "->" + currency
        return money.amount * exchange_rates[key]
