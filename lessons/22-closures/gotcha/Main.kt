fun main() {
    var count = 0
    val inc = { count++ }
    inc()
    inc()
    println("count=$count")
}
