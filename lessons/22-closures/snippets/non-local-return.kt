fun firstEven(nums: List<Int>): Int? {
    nums.forEach { if (it % 2 == 0) return it }   // returns from firstEven
    return null
}

fun main() {
    println("firstEven([3, 5, 8, 9]): ${firstEven(listOf(3, 5, 8, 9))}")
    println("firstEven([3, 5]): ${firstEven(listOf(3, 5))}")
}
