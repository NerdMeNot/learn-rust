fun total(xs: List<Int>): Int = xs.sum()

fun main() {
    val xs = mutableListOf(1, 2, 3)
    val view: List<Int> = xs
    xs.add(4)
    println("$view ${total(view)}")
    (view as MutableList<Int>).add(5)
    println(xs)
}
