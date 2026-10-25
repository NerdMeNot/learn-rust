class Noisy(val name: String) : AutoCloseable {
    override fun close() = println("  close $name")
}

fun main() {
    Noisy("a").use { a ->
        Noisy("b").use { b ->
            println("  end of block")
        }
    }
}
