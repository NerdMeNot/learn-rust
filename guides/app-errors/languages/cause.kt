fun main() {
    try {
        try {
            "eighty".toInt()
        } catch (e: NumberFormatException) {
            throw RuntimeException("reading bad.conf", e)
        }
    } catch (e: RuntimeException) {
        println("${e.message}: ${e.cause?.message}")
    }
    // runCatching turns a thrown exception into a Result value
    val r = runCatching { "eighty".toInt() }
    println("runCatching: isFailure=${r.isFailure}")
}
