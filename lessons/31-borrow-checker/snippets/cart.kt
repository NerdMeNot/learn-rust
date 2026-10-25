class Cart {
    private val items = mutableListOf<String>()

    fun items(): List<String> = items

    fun size() = items.size
}

fun main() {
    val cart = Cart()
    val view = cart.items()
    (view as MutableList<String>).add("tea")
    println("cart size: ${cart.size()}")
}
