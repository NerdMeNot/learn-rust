interface Shape {
    fun name(): String
    fun area(): Int
}

class Square(val side: Int) : Shape {
    override fun name() = "square"
    override fun area() = side * side
}

class Rect(val w: Int, val h: Int) : Shape {
    override fun name() = "rect"
    override fun area() = w * h
}

fun main() {
    val shapes: List<Shape> = listOf(Square(3), Rect(4, 6))

    var total = 0
    for (s in shapes) {
        println("${s.name()}: ${s.area()}")
        total += s.area()
    }
    println("total: $total")
}
