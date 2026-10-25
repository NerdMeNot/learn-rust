fun totalWithTax(price: Int): Int {
    val rate = 20
    fun tax(p: Int) = p * rate / 100 // a local function may use `rate`
    return price + tax(price)
}

fun area(w: Int, h: Int = 1) = w * h

fun sum(vararg xs: Int) = xs.sum()

fun nothing() {}

fun twice(n: Int) = n * 2

fun fill(xs: MutableList<String>) {
    xs.add("x")
}

fun todoLater(): Nothing = TODO("later")

fun depth(n: Long): Long = if (n == 0L) 0 else 1 + depth(n - 1)

fun later() = 1

fun main() {
    check(totalWithTax(50) == 60) { "local function captures" }
    check(area(3) == 3 && area(3, 4) == 12 && area(w = 2, h = 5) == 10) { "default and named" }
    check(sum(1, 2, 3) == 6) { "vararg" }
    check(nothing() == Unit) { "Unit" }
    val (q, r) = Pair(7 / 2, 7 % 2)
    check(q == 3 && r == 1) { "Pair" }
    val f: (Int) -> Int = ::twice
    check(f(4) == 8) { "function reference" }
    val xs = mutableListOf<String>()
    fill(xs)
    check(xs.size == 1) { "the caller sees the callee's change" }
    val g: () -> Nothing = ::todoLater
    check(try { g(); false } catch (e: NotImplementedError) { true }) { "TODO" }
    val deep = try { depth(100_000_000L); false } catch (e: StackOverflowError) { true }
    check(deep) { "StackOverflowError is catchable" }
    println("all checks passed")
}
