class Account:
    def __init__(self):
        self.balance = 0

    def deposit(self, amount):
        self.balance += amount


acct = Account()
acct.deposit(5)
Account.deposit(acct, 5)  # the same call, written out
print("balance:", acct.balance)
