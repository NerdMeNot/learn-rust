suspend fun loadCount(): Int = 3

suspend fun report(): Int {
    val n = loadCount() // a suspend call looks like a plain call
    return n + 1
}

suspend fun main() {
    println("count: ${report()}")
}
