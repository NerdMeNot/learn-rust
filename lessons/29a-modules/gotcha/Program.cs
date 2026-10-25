Console.WriteLine($"total: {Shop.Total(12)}");
Console.WriteLine($"discount: {Shop.DiscountPercent(12)}%");

static class Shop
{
    // No modifier on a member means private.
    static int DiscountPercent(int qty) => qty >= 10 ? 20 : 0;

    public static int Total(int qty)
    {
        int full = qty * 10;
        return full - full * DiscountPercent(qty) / 100;
    }
}
