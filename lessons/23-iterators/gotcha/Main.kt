fun main() {
    val nums = listOf(1, 2, 3, 4, 5, 6)
    val evens = nums.filter { n -> n % 2 == 0 }
    println(evens)
}
