object Shop {
    private fun discountPercent(qty: Int): Int = if (qty >= 10) 20 else 0

    fun total(qty: Int): Int {
        val full = qty * 10
        return full - full * discountPercent(qty) / 100
    }
}

fun main() {
    println("total: ${Shop.total(12)}")
    println("discount: ${Shop.discountPercent(12)}%")
}
