fun main() {
    val a = mutableListOf(1, 2, 3)
    val b = a
    b.add(4)
    println(a)
    println(a === b)
}
