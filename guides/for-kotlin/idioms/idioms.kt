data class Pt(val x: Int, val y: Int) {
    companion object {
        fun origin() = Pt(0, 0)
    }
}

sealed class Shape
class Circle(val r: Double) : Shape()
object Dot : Shape()

val answer by lazy { 6 * 7 }

fun f(s: String): Int = s.toInt()

fun main() {
    val xs = listOf(3, 1, 2)
    println("2 ${xs.map { it * 2 }}")
    println("3 ${xs.filter { it > 1 }.sum()}")
    val sb = StringBuilder()
    for ((i, x) in xs.withIndex()) sb.append("$i:$x ")
    println("4 ${sb.toString().trim()}")
    val name = "ana"
    val age = 30
    val s = "$name is $age"
    println("5 $s")
    println("6 ${"x".toIntOrNull() ?: 0}")
    val m = mutableMapOf<String, MutableList<Int>>()
    m.getOrPut("a") { mutableListOf() }.add(1)
    println("7 ${m["a"]}")
    val pair = Pair(1, "one")
    val (a, b) = pair
    println("8 $a $b")
    val o: Shape = Circle(2.0)
    println("9 ${if (o is Circle) o.r else 0.0}")
    val x: Int? = 4
    println("10 ${x?.let { it + 1 }}")
    println("11 ${xs.firstOrNull { it > 1 }}")
    val ys = mutableListOf("bb", "a", "ccc")
    ys.sortBy { it.length }
    println("12 ${ys.joinToString(" ")}")
    val p = Pt(1, 2)
    val q = p.copy(x = 5)
    println("13 ${q.x},${q.y}")
    println("14 ${Pt.origin().x},${Pt.origin().y}")
    println("15 $answer")
    val n = try { f("x") } catch (e: NumberFormatException) { 0 }
    println("16 $n")
    val words = listOf("a", "b", "c")
    println("17 ${words.joinToString(", ")}")
    val r1 = StringBuilder()
    for (i in 1 until 3) r1.append(i)
    val r2 = StringBuilder()
    for (i in 1..3) r2.append(i)
    println("18 $r1 $r2")
    val k = 2
    val size = when (k) { 1, 2 -> "low"; else -> "high" }
    println("19 $size")
    println("22 ${"kotlin".substring(1, 3)}")
}
