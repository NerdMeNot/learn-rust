object Shop {
    private const val PRICE = 10

    // private: only code inside `Shop` can call this
    private fun discountPercent(qty: Int): Int = if (qty >= 10) 20 else 0

    fun total(qty: Int): Int {
        val full = qty * PRICE
        return full - full * discountPercent(qty) / 100
    }
}

fun main() {
    println("3 items: ${Shop.total(3)}")
    println("12 items: ${Shop.total(12)}")
}
