sealed interface Shape
class Circle(val r: Int) : Shape
class Rect(val w: Int, val h: Int) : Shape

fun area(s: Shape): Int = when (s) {
    is Circle -> 3 * s.r * s.r
    is Rect -> s.w * s.h
}

fun main() {
    println(area(Circle(2)))
    println(area(Rect(2, 5)))
}
