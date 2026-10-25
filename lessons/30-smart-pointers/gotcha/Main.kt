class Counter(var hits: Int)

fun main() {
    val a = Counter(0)
    val b = a

    a.hits++
    b.hits++
    println("hits: ${a.hits}")
}
