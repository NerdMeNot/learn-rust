if (int.TryParse("4x", out _)) throw new Exception("TryParse accepted 4x");
if (!int.TryParse("42", out var n) || n != 42) throw new Exception("TryParse");
if (new Money(150) + new Money(250) != new Money(400)) throw new Exception("operator +");
if (new Money(1) != new Money(1)) throw new Exception("record equality");
if ((new Money(1) with { Cents = 5 }) != new Money(5)) throw new Exception("with");
if ((object)new Money(1) is IComparable) throw new Exception("a record is not IComparable");
if (!double.NaN.Equals(double.NaN) || double.NaN == double.NaN) throw new Exception("NaN");
Console.WriteLine("all checks passed");

record Money(int Cents)
{
    public static Money operator +(Money a, Money b) => new Money(a.Cents + b.Cents);
}
