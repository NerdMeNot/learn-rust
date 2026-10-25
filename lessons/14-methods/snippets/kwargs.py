class Account:
    def __init__(self, owner, balance=0, currency="USD"):
        self.owner = owner
        self.balance = balance
        self.currency = currency


a = Account("ana", currency="EUR")
print(a.owner, a.balance, a.currency)
