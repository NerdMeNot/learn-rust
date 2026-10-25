// Backs the Kotlin cells of the table in lesson.mdx. Never shown.
fun must(ok: Boolean, what: String) {
    if (!ok) throw AssertionError(what)
}

fun <T> bigger(a: T, b: T): T where T : Comparable<T>, T : java.io.Serializable = if (a > b) a else b

fun total(xs: MutableList<out Number>): Double = xs.sumOf { it.toDouble() }

fun <T> describe(x: T): String = "${x.toString()} ${x == x} ${x.hashCode()}"

class Pair<T>(val first: T, val second: T) {
    fun <R> map(f: (T) -> R): Pair<R> = Pair(f(first), f(second))
}

fun main() {
    must(describe(7) == "7 true 7", "unbounded T")
    must(Pair(1, 2).map { "#$it" }.first == "#1", "generic method")
    must(emptyList<String>().isEmpty(), "explicit type argument")
    must(bigger("a", "b") == "b", "two bounds")
    val ints: List<Int> = listOf(1, 2)
    must(ints.sum() == 3, "primitive type argument")
    val doubles: MutableList<Double> = mutableListOf(1.5, 2.5)
    must(total(doubles) == 4.0, "out projection")
    println("ok")
}
