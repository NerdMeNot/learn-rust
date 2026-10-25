sealed class Shape {
    data class Circle(val r: Double) : Shape()
    data class Rect(val w: Double, val h: Double) : Shape()
    data class Triangle(val b: Double, val h: Double) : Shape() // new!
}

fun area(s: Shape): Double = when (s) {
    is Shape.Circle -> 3.14 * s.r * s.r
    is Shape.Rect -> s.w * s.h
    else -> 0.0 // compiles, and now covers Triangle too
}

fun main() {
    println("triangle area: ${area(Shape.Triangle(3.0, 4.0))}")
}
