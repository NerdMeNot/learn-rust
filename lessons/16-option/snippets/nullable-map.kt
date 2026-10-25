fun main() {
    val stock: Map<String, Int?> = mapOf("apple" to 3, "pear" to null)

    // T? is T-or-null, so "key holds null" and "no such key" look the same.
    println("stock[apple]: ${stock["apple"]}")
    println("stock[pear]: ${stock["pear"]}")
    println("stock[plum]: ${stock["plum"]}")
    println("pear in stock: ${"pear" in stock}")
}
