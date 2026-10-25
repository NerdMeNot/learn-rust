// returns a piece of the input: a view of it, not a copy
fun firstTwo(xs: List<Int>): List<Int> = xs.subList(0, 2)

fun main() {
    val scores = mutableListOf(3, 9, 4)
    val top = firstTwo(scores)
    println("top: $top")
    scores.add(7) // the owner changes the list while the view exists
    println("top: $top")
}
