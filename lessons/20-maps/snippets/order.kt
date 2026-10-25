fun main() {
    val m = mutableMapOf("one" to 1, "two" to 2, "three" to 3, "four" to 4, "five" to 5, "six" to 6)
    println("mutableMapOf: ${m.keys} is a ${m::class.java.name}")

    val h = hashMapOf("one" to 1, "two" to 2, "three" to 3, "four" to 4, "five" to 5, "six" to 6)
    println("hashMapOf: ${h.keys} is a ${h::class.java.name}")

    val counts = mutableMapOf<String, Int>()
    for (word in "a b a".split(" ")) {
        counts.merge(word, 1, Int::plus)
    }
    println("counts: $counts")
}
