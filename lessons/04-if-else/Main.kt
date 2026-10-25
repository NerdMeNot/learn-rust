fun main() {
    val n = 7
    if (n % 2 == 0) {
        println("$n is even")
    } else {
        println("$n is odd")
    }

    val size = if (n > 5) "big" else "small"
    println(size)
}
