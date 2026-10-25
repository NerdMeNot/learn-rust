fun must(ok: Boolean, what: String) {
    if (!ok) throw AssertionError(what)
}

fun main() {
    val v = mutableListOf(3, 1, 2)
    v.add(4)
    must(v.removeLastOrNull() == 4, "removeLastOrNull")
    must(mutableListOf<Int>().removeLastOrNull() == null, "removeLastOrNull on empty")
    must(v.removeAt(1) == 1 && v == listOf(3, 2), "removeAt")
    v.add(1, 5)
    must(v == listOf(3, 5, 2) && v.last() == 2 && v.lastOrNull() == 2, "add last")
    must(v.getOrNull(99) == null && v.size == 3 && 5 in v, "getOrNull size in")
    v.addAll(listOf(7, 8))
    v.removeAll { it == 7 }
    must(v == listOf(3, 5, 2, 8), "removeAll")
    must(v.subList(1, 3) == listOf(5, 2), "subList")
    v.sort()
    must(v == listOf(2, 3, 5, 8), "sort")
    v.sortBy { -it }
    must(v == listOf(8, 5, 3, 2), "sortBy")
    val words = mutableListOf("kiwi", "fig", "banana")
    words.sortWith(compareBy { it.length })
    must(words == listOf("fig", "kiwi", "banana"), "sortWith")
    val ro: List<Int> = v
    must(ro.toMutableList() == v && ro.sorted() == listOf(2, 3, 5, 8), "toMutableList sorted")
    val sub = v.subList(1, 3)
    v[1] = 42
    must(sub[0] == 42, "subList is a view")
    println("all checks passed")
}
