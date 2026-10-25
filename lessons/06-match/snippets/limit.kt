fun main() {
    val limit = 10
    for (n in listOf(10, 3)) {
        val text = when (n) {
            limit -> "at the limit"
            else -> "under"
        }
        println("$n: $text")
    }
}
