const val MAX = 100

fun add(a: Int, b: Int): Int { return a + b }

fun must(ok: Boolean, what: String) {
    if (!ok) throw AssertionError(what)
}

fun main() {
    val count: Int = 5
    var mutable = 5
    mutable += count
    must(mutable == 10, "val var")

    val pair = Pair(1, "two")
    val (a, b) = pair
    must(a == 1 && b == "two", "destructuring")

    val n = 7
    val d = 2.9
    must(n.toDouble() == 7.0 && d.toInt() == 2 && (-d).toInt() == -2, "conversion")

    val inc = { x: Int -> x + 1 }
    val inc2: (Int) -> Int = { it + 1 }
    must(inc(1) == 2 && inc2(1) == 2 && add(1, 2) == 3 && MAX == 100, "lambda")

    val typed = emptyList<String>()
    val ints = listOf<Int>()
    must(typed.isEmpty() && ints.isEmpty(), "explicit type arguments")

    must("42".toInt() == 42, "toInt on text")
    val raw = """C:\temp"""
    must(raw.length == 7, "raw string")

    val c: Char = 'R'
    must(c.code == 82 && Char.SIZE_BYTES == 2, "char is 2 bytes")
    must(1_000 == 1000 && 0xFF == 255 && 0b101 == 5, "literals")
    val big = 5L
    val f = 2.5f
    must(big == 5L && f == 2.5f, "suffixes")
}
