class Point(var x: Int, var y: Int) {
    override fun toString() = "($x, $y)"
}

fun main() {
    val a = Point(1, 2)
    val b = a // b is another name for the same object
    b.x = 99
    println("a = $a")
    println("b = $b")

    // To get a separate point, you build one.
    val c = Point(a.x, a.y)
    c.x = 7
    println("a after c.x = 7: $a")
}
