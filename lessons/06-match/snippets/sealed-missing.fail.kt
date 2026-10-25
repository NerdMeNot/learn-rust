sealed interface Shape
class Circle(val r: Int) : Shape
class Rect(val w: Int, val h: Int) : Shape

fun area(s: Shape): Int = when (s) {
    is Circle -> 3 * s.r * s.r
}

fun main() {
    println(area(Circle(2)))
}
