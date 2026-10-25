fun main() {
    val x = 1
    run {
        val x = 10
        println("inner x: $x")
    }
    println("outer x: $x")
}
