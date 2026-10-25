data class Id(val n: Int) {
    override fun toString() = "#$n"
}

fun main() {
    println("[%8s] [%-8s]".format(Id(42), Id(42)))
    println("%.0f %.0f %.0f".format(0.5, 1.5, 2.5))
}
