fun <T : Comparable<T>> largest(items: List<T>): T {
    var best = items[0]
    for (item in items) {
        if (item > best) {
            best = item
        }
    }
    return best
}

fun main() {
    println(largest(listOf(3, 9, 2)))
    println(largest(listOf("kiwi", "apple", "pear")))
}
