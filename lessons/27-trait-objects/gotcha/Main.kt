interface Shape {
    fun area(): Int
}

class Square(val side: Int) : Shape {
    override fun area() = side * side
}

class Rect(val w: Int, val h: Int) : Shape {
    override fun area() = w * h
}

fun main() {
    val shapes: List<Shape> = listOf(Square(3), Rect(4, 6))

    println("total: ${shapes.sumOf { it.area() }}")
}
