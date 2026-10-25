fun sum(vararg xs: Int): Int = xs.sum()

fun main() {
    println("sum() = ${sum()}")
    println("sum(1, 2, 3, 4) = ${sum(1, 2, 3, 4)}")
}
