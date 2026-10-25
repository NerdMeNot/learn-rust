class Account {
  balance = 0;

  deposit(amount: number): void {
    this.balance += amount;
  }
}

const acct = new Account();
acct.deposit(5);
console.log("after acct.deposit(5):", acct.balance);

const f = acct.deposit; // the method, without the object
try {
  f(5); // `this` is gone
} catch (e) {
  console.log("f(5) failed:", (e as Error).message);
}
