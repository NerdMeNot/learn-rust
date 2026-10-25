fun minMax(xs: List<Int>): Pair<Int, Int> {
    var lo = xs[0]
    var hi = xs[0]
    for (x in xs) {
        if (x < lo) lo = x
        if (x > hi) hi = x
    }
    return Pair(lo, hi)
}

fun main() {
    val (lo, hi) = minMax(listOf(4, 9, 1, 7))
    println("$lo $hi")

    val (_, onlyHi) = minMax(listOf(3, 5))
    println(onlyHi)
}
