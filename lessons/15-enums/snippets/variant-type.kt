sealed class Shape {
    data class Circle(val r: Double) : Shape()
    data class Rect(val w: Double, val h: Double) : Shape()
}

// each case is its own type, so a function can ask for exactly one
fun radius(c: Shape.Circle): Double = c.r

fun main() {
    println("radius: ${radius(Shape.Circle(1.0))}")
}
