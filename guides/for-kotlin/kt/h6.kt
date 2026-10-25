sealed class Shape
class Circle(val r: Double) : Shape()
class Rect(val w: Double, val h: Double) : Shape()

fun area(s: Shape): Double = when (s) {
    is Circle -> 3.0 * s.r * s.r
    is Rect -> s.w * s.h
}

fun circleArea(c: Circle): Double = area(c)

fun main() {
    println(area(Rect(2.0, 3.0)))
    println(circleArea(Circle(1.0)))
}
