data class Point(val x: Int, val y: Int)

fun describe(o: Any): String = when {
    o is String && o.length > 3 -> "long text ${o.length}"
    o is Point -> "point at ${o.x}"
    else -> "other"
}

fun main() {
    val (y, x) = Point(1, 2)
    println("x=$x y=$y")
    println(describe("hello"))
    println(describe(Point(5, 6)))
    println(describe(3.5))
}
