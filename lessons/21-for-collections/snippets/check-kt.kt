// Backs the Kotlin cells of the tables in lesson.mdx. Never shown.
fun main() {
    val v = listOf("ana", "bo", "cy")
    check(v.withIndex().map { "${it.index}${it.value}" } == listOf("0ana", "1bo", "2cy"))
    check(v.zip(listOf(31, 27)).size == 2)
    check(v.reversed() == listOf("cy", "bo", "ana"))
    check(listOf(1, 2, 3, 4, 5).windowed(2).size == 4)
    check(listOf(1, 2, 3, 4, 5).chunked(2).last() == listOf(5))
    val m = mutableListOf(1, 2, 3, 4)
    m.removeAll { it % 2 == 0 }
    check(m == listOf(1, 3))
    val l = mutableListOf(1, 2, 3, 4)
    try {
        for (x in l) {
            if (x == 2) l.add(99)
        }
        error("no exception")
    } catch (e: ConcurrentModificationException) {
        // expected
    }
    println("ok")
}
