fun must(ok: Boolean, what: String) {
    if (!ok) throw IllegalStateException(what)
}

fun main() {
    var hits = 0
    outer@ for (a in 0 until 3) {
        for (b in 0 until 3) {
            if (b == 1) continue@outer
            if (a == 2) break@outer
            hits++
        }
    }
    must(hits == 2, "labels")

    var n = 0
    do { n += 5 } while (n < 3)
    must(n == 5, "do-while runs once")

    for ((i, x) in listOf("a", "b").withIndex()) must(x == listOf("a", "b")[i], "withIndex")
    // 4..0 is empty in Kotlin too; downTo counts down
    must((4..0).toList().isEmpty(), "4..0 is empty")
    must((4 downTo 0).toList() == listOf(4, 3, 2, 1, 0), "downTo")
    println("ok")
}
