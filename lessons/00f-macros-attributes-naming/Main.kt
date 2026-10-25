/** A point on a grid. */
data class Point(val x: Int, val y: Int)

fun main() {
    val p = Point(3, 4) // a line comment
    val items = listOf(1, 2, 3)
    check(items.size == 3) // stops with an exception if false
    println("point: Point { x: ${p.x}, y: ${p.y} }")
    println("items: $items")
    println("row: [${"%5d".format(42)}] [${"%-5d".format(42)}] [${"%05d".format(42)}]")
    val pi = "%.2f".format(3.14159)
    println("pi: $pi {done}")
}
