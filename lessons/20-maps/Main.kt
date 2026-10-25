fun main() {
    val ages = mutableMapOf<String, Int>()
    ages["ana"] = 30
    ages["bo"] = 25

    val ana = ages["ana"]           // Int? : null when missing
    println(if (ana != null) "ana is $ana" else "no ana")

    val zed = ages["zed"] ?: 0
    println("zed is $zed")

    val counts = mutableMapOf<String, Int>()
    for (word in "a b a c a".split(" ")) {
        counts[word] = (counts[word] ?: 0) + 1
    }
    println("a appears ${counts["a"]} times")
}
