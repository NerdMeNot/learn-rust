class Counter {
    var count = 0
    fun bump() { count++ }
}

fun main() {
    val c = Counter()
    c.bump()
    println(c.count)
}
