class Money(val cents: Long) {
    override fun toString() = "\$${cents / 100}.${"%02d".format(cents % 100)}"
}

data class Item(val name: String, val qty: Int)

fun main() {
    val n = 42
    println("[%6d] [%-6d] [%06d]".format(n, n, n))
    println("%.2f %e %#x".format(3.14159, 1234.5, n) + " 0b" + n.toString(2))

    val price = Money(1999)
    println("total: $price")

    val label = "$price each"
    println("$label (${label.length} chars)")

    val item = Item("pen", 3)
    println("${item.qty} x ${item.name}")
    println(item)
}
