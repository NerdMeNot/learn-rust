fun parseAge(s: String): Int {
    return s.toInt()       // throws NumberFormatException on bad input
}

fun main() {
    for (s in listOf("42", "abc")) {
        try {
            println("age ${parseAge(s)}")
        } catch (e: NumberFormatException) {
            println("error: ${e.message}")
        }
    }
}
