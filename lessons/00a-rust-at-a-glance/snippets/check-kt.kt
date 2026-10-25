fun must(ok: Boolean, what: String) {
    if (!ok) throw AssertionError(what)
}

fun add(a: Int, b: Int): Int = a + b

fun main() {
    var x = 5
    x = 6
    val y = 5
    must(x == 6 && y == 5, "variables")
    must(add(2, 3) == 5, "fun")

    val seen = mutableListOf<Int>()
    for (i in 0 until 3) seen.add(i)
    must(seen == listOf(0, 1, 2), "until")
    val inclusive = mutableListOf<Int>()
    for (i in 0..3) inclusive.add(i)
    must(inclusive == listOf(0, 1, 2, 3), "0..3 includes 3")

    val words = listOf("a", "b")
    var joined = ""
    for (w in words) joined += w
    must(joined == "ab", "for in")

    val n = 1
    val s = when (n) {
        1 -> "one"
        else -> "many"
    }
    must(s == "one", "when")

    val inc = { v: Int -> v + 1 }
    must(inc(1) == 2, "lambda")

    val name = "Ana"
    val age = 30
    println("$name is $age")

    var threw = false
    try {
        "x".toInt()
    } catch (e: NumberFormatException) {
        threw = true
    }
    must(threw, "toInt throws")
    println("all kotlin cells compile and hold")
}
