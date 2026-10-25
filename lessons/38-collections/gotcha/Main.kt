fun main() {
    val counts = mutableMapOf<String, Int>()
    counts["cat"] = 0
    for (w in "cat cat".split(" ")) {
        counts[w] += 1
    }
    println(counts)
}
