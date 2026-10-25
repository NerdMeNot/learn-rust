fun main() {
    var n = 0
    for (i in 0 until 5) {
        when (i) {
            2 -> break
            else -> n++
        }
    }
    println("n: $n")
}
