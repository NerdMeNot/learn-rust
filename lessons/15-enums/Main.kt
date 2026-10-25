sealed class Shape {
    data class Circle(val r: Double) : Shape()
    data class Rect(val w: Double, val h: Double) : Shape()
}

fun area(s: Shape): Double = when (s) {
    is Shape.Circle -> 3.14 * s.r * s.r
    is Shape.Rect -> s.w * s.h
}

fun main() {
    val shapes = listOf(Shape.Circle(1.0), Shape.Rect(2.0, 3.0))
    for (s in shapes) {
        println(area(s))
    }
}
