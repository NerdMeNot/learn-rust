data class Point(val x: Int, val y: Int)

fun main() {
    // a destructuring declaration is by position: the names do not have to match the properties
    val (y, x) = Point(1, 2)
    println("y = $y, x = $x")

    // destructuring copies references out: the source stays whole
    val p = Point(5, 6)
    val (px, py) = p
    println("p is still $p (px=$px, py=$py)")
}
