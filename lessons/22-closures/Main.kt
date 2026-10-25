fun main() {
    val factor = 3
    val times = { x: Int -> x * factor }
    println(times(5))

    var count = 0
    val inc = { count++ }
    inc()
    inc()
    println("count=$count")
}
