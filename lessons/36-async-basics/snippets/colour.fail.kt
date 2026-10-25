suspend fun readCount(): Int = 3

fun total(): Int = readCount() + 1

fun main() {
    println(total())
}
