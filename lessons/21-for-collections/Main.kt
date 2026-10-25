fun main() {
    val names = listOf("ana", "bo")
    for (n in names) {
        println("hi $n")
    }

    val nums = mutableListOf(1, 2, 3)
    for (i in nums.indices) {
        nums[i] *= 10
    }

    for ((i, n) in nums.withIndex()) {
        println("$i: $n")
    }
    println("${names.size} names, $nums")
}
