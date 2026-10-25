Console.WriteLine($"3 items: {Shop.Total(3)}");
Console.WriteLine($"12 items: {Shop.Total(12)}");

static class Shop
{
    const int Price = 10;

    // Members are private unless you say otherwise.
    static int DiscountPercent(int qty) => qty >= 10 ? 20 : 0;

    public static int Total(int qty)
    {
        int full = qty * Price;
        return full - full * DiscountPercent(qty) / 100;
    }
}
