fun describe(x: Any): String {
    if (x is String) {
        return "text of length " + x.length
    }
    return "something else"
}

fun main() {
    println(describe("hello"))
    println(describe(42))
    val n = 7
    val size = if (n > 5) "big" else "small"
    println(size)
}
