fun <T : Comparable<T>> largest(list: List<T>): T {
    var best = list[0]
    for (item in list) {
        if (item > best) {
            best = item
        }
    }
    return best
}

fun main() {
    val numbers = listOf(3, 9, 4)
    val words = listOf("pear", "fig", "apple")
    println("largest number: ${largest(numbers)}")
    println("largest word: ${largest(words)}")
}
