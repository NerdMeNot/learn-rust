interface Shape {
    fun area(): Int
}

class Square(val side: Int) : Shape {
    override fun area() = side * side
}

class Rect : Shape {
    override fun area() = 24
}

sealed interface Kind
class Circle(val r: Int) : Kind
class Tri(val b: Int) : Kind

abstract class Base {
    protected val id = 7
    abstract fun area(): Int
}

interface Named {
    val name: String
}

class Dog : Named {
    override val name = "dog"
}

fun <T : Shape> twice(s: T) = 2 * s.area()

fun kind(k: Kind) = when (k) {
    is Circle -> "circle ${k.r}"
    is Tri -> "tri ${k.b}"
}

fun main() {
    val shapes: List<Shape> = listOf(Square(3), Rect())
    // `is` with a smart cast
    val first = shapes[0]
    check(first is Square && first.side == 3)
    check(shapes[1] !is Square)
    // sealed interface + when
    check(kind(Circle(2)) == "circle 2" && kind(Tri(5)) == "tri 5")
    // a nullable list element
    val maybe: List<Shape?> = listOf(Square(1), null)
    check(maybe[1] == null)
    // an abstract class can hold a field
    val b = object : Base() { override fun area() = id }
    check(b.area() == 7)
    // a generic bound
    check(twice(Square(3)) == 18)
    // an interface property (not a field)
    check(Dog().name == "dog")
    println("ok")
}
