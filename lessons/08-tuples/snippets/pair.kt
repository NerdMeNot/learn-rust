data class Size(val width: Int, val height: Int)

fun main() {
    val p = Pair(3, 4)
    println("p.first: ${p.first}, p.second: ${p.second}")
    val (a, b) = p
    println("a: $a, b: $b")
    val t = Triple(1, "two", true)
    println("t.third: ${t.third}")
    println("1 to 2: ${1 to 2}")
    val (w, h) = Size(16, 9)
    println("data class: w: $w, h: $h")
    println("equal: ${Pair(3, 4) == p}, class: ${p.javaClass.name}")
    var x = 1
    var y = 2
    x = y.also { y = x }
    println("swap: $x $y")
}
