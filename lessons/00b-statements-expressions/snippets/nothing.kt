fun main() {
    val ok = true
    val x: Int = if (ok) 5 else throw IllegalStateException("not ok")
    println("x: $x")
    val area = { 3 * 4 }
    println("area is a lambda, so call it: ${area()}")
}
