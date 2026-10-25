fun parseAge(s: String): Int = s.trim().toInt()

fun total(a: String, b: String): Int = parseAge(a) + parseAge(b)

fun main() {
    println(total("4", "5"))
    try {
        println(total("4", "x"))
    } catch (e: NumberFormatException) {
        println("caught: ${e.message}")
    }
    println(runCatching { total("4", "x") }.isFailure)
}
