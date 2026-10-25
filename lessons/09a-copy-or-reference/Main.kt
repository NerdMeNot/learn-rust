fun main() {
    val ages = mutableListOf(30, 40, 50)

    // A copy: the loop variable is a val holding its own number.
    for (age in ages) {
        val copy = age + 1
        println("the copy is now $copy")
    }
    println("ages after the copy loop:     $ages")

    // The elements themselves: assign through the index.
    for (i in ages.indices) {
        ages[i]++
    }
    println("ages after the in-place loop: $ages")
}
