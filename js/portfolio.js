const Money = require('./money');

class Portfolio {
    constructor() {
        this.moneys = [];
    }

    add(...moneys) {
        this.moneys = this.moneys.concat(moneys);
    }

    evaluate(currency) {
        let total = this.moneys.reduce((sum, money) => {
            return sum + this.convert(money, currency);
        }, 0);
        return new Money(total, currency);
    }

    convert(money, currency) {
        if (money.currency === currency) {
            return money.amount;
        }
        let eurToUsd = 1.2;
        return money.amount * eurToUsd;
    }
}

module.exports = Portfolio;
