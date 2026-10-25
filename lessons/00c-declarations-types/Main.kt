const val LIMIT = 1_000

fun half(x: Double): Double = x / 2.0

fun main() {
    val name: String = "Ada"
    var visits: Byte = 0
    visits++
    val (mask, ratio) = Pair(0xff, 2.5f)
    val letter = 'R'
    val total = LIMIT.toDouble() + half(5.0)
    val parsed = "42".toInt()

    println("name=$name visits=$visits")
    println("mask=$mask ratio=$ratio letter=$letter")
    println("limit=$LIMIT total=$total")
    println("parsed=$parsed")
}
