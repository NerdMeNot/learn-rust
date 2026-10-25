class Counter {
    private var count = 0

    fun get(): Int = count

    fun bump() {
        count++
    }
}

fun main() {
    val c = Counter()
    c.bump()
    c.bump()
    println(c.get())
}
