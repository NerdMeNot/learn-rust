// What other languages write with ?:, i++, null and switch, written in Kotlin.
fun size(n: Int): String = when (n) {
    1, 2 -> "small"
    3 -> "medium"
    else -> "large"
}

fun firstEven(items: List<Int>): Int? {
    for (x in items) {
        if (x % 2 == 0) return x
    }
    return null
}

fun main() {
    var total = 0
    for (i in 0 until 4) {
        total += i
    }
    val label = if (total > 5) "big" else "small"
    println("total: $total ($label)")
    println("size: ${size(2)} ${size(9)}")
    val even = firstEven(listOf(3, 5, 8))
    if (even != null) println("first even: $even") else println("no even number")
    println("none: ${firstEven(listOf(1, 3)) ?: -1}")
    val wide = total.toLong() * 1_000_000_000
    println("wide: $wide")
}
