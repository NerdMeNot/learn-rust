var a = new Account("ana", currency: "EUR");
Console.WriteLine($"{a.Owner} {a.Balance} {a.Currency}");

class Account
{
    public string Owner;
    public int Balance;
    public string Currency;

    public Account(string owner, int balance = 0, string currency = "USD")
    {
        Owner = owner;
        Balance = balance;
        Currency = currency;
    }
}
