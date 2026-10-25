fun main() {
    // every object is already a shared reference
    val log = mutableListOf<String>()
    val other = log

    log.add("from log")
    other.add("from other")

    println("log: ${log.joinToString(", ")}")
    println("same list: ${log === other}")
}
