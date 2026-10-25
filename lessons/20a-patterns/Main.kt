data class Point(val x: Int, val y: Int)

fun main() {
    // destructuring declaration: by position, through component1() and component2()
    val (x, y) = Point(3, -2)
    println("x = $x, y = $y")

    for ((i, c) in "abc".withIndex()) {
        println("$i: $c")
    }

    val kv = "mode=fast".split("=", limit = 2)
    if (kv.size == 2) {
        val (key, value) = kv
        println("$key -> $value")
    }

    // a List destructures by position too; _ skips an item
    val (first, _, _, last) = listOf(10, 20, 30, 40)
    println("first = $first, last = $last")
}
