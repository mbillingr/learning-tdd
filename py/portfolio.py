from money import Money


class Portfolio:
    def __init__(self):
        self.moneys = []
        self._eur_to_usd = 1.2

    def add(self, *moneys):
        self.moneys.extend(moneys)

    def evaluate(self, bank, currency):
        total = 0
        failures = []
        for m in self.moneys:
            try:
                total += bank.convert(m, currency).amount
            except Exception as ex:
                failures.append(ex)

        if not failures:
            return Money(total, currency)

        failure_message = ",".join(f.args[0] for f in failures)
        raise Exception("Missing exchange rate(s): [" + failure_message + "]")
