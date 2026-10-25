fun place(p: Pair<Int, Int>): String = when (p) {
    Pair(0, 0) -> "origin"
    else -> {
        val (x, y) = p
        if (x == 0) "on the y axis, y = $y" else "elsewhere"
    }
}

fun main() {
    println(place(Pair(0, 0)))
    println(place(Pair(0, 7)))
    println(place(Pair(3, 4)))
}
