fun main() {
    val a: Int = 7
    val b: Long = 10
    val price = 2.5
    val ok = true

    val total = a + b        // Kotlin widens Int to Long for you
    val cost = a * price     // and Int to Double
    println("$total $cost $ok")
}
