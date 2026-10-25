class Span(private val name: String) : AutoCloseable {
    init { println("enter $name") }
    override fun close() { println("exit $name") }
}

fun main() {
    Span("request").use { println("listed notes") } // close() runs when the block ends
    Span("forgotten") // nothing ever calls close()
    println("after")
}
