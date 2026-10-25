fun main() {
    val nums = listOf(1, 2, 3, 4, 5, 6)

    val evensSquared = nums
        .filter { it % 2 == 0 }
        .map { it * it }

    val total = nums.sum()
    println("$evensSquared $total")
}
