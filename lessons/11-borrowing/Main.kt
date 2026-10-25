fun total(xs: List<Int>): Int {
    var sum = 0
    for (x in xs) {
        sum += x
    }
    return sum
}

fun main() {
    val nums = listOf(1, 2, 3)
    val t = total(nums)
    println("$t $nums")
}
