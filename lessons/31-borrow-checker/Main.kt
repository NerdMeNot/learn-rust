class Item(val name: String, val count: Int)

class Inventory(val items: List<Item>) {
    val log = mutableListOf<String>()

    fun check() {
        for (item in items) {
            if (item.count == 0) {
                log.add("out of stock: ${item.name}")
            }
        }
    }
}

fun main() {
    val inv = Inventory(listOf(Item("bolt", 0), Item("nut", 12), Item("washer", 0)))
    inv.check()
    println("log: [" + inv.log.joinToString(", ") { "\"$it\"" } + "]")
}
