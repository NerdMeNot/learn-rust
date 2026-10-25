fun main() {
    // mapOf is an ordinary function; `to` makes a pair
    val ages = mapOf("Ada" to 36, "Linus" to 28, "Grace" to 45)

    for (name in ages.keys.sorted()) {
        println("$name: ${ages[name]}")
    }
    println("entries: ${ages.size}")
}
