// Greet someone, then add up 1 to 4.
fun greet(name: String): String = "Hello, $name!"

fun add(a: Int, b: Int): Int = a + b

fun main() {
    println(greet("Ferris"))
    var total = 0
    for (n in 1..4) {
        total = add(total, n)
    }
    println("total: $total")
}
