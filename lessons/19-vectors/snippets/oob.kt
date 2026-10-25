fun main() {
    val list = mutableListOf(10, 20, 30)
    try {
        list[10]
    } catch (e: IndexOutOfBoundsException) {
        println("list[10]: $e")
    }
    println("list.getOrNull(10): ${list.getOrNull(10)}")
}
