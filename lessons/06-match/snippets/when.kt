fun classify(n: Int): String = when (n) {
    0 -> "zero"
    in 1..5 -> "few"
    6, 7 -> "some"
    else -> if (n < 0) "negative" else "many"
}

fun size(n: Int): String = when {
    n > 100 -> "big"
    n > 10 -> "medium"
    else -> "small"
}

fun main() {
    for (n in listOf(0, 3, 5, 7, 12, -1)) {
        println("$n -> ${classify(n)}")
    }
    println("${size(500)} ${size(50)} ${size(5)}")
}
