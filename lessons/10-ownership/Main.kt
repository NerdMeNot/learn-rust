fun main() {
    val a = mutableListOf(1, 2, 3)
    val b = a        // a and b point at the SAME list
    b[0] = 100
    println(a)       // [100, 2, 3]
    println(b)

    val x = 5
    val y = x
    println("$x $y")
}
