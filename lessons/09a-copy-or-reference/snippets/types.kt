data class Point(var x: Int)

data class Team(val name: String, val members: MutableList<String>)

fun report(kind: String, changed: Boolean) {
    println("%-8s %s".format(kind, if (changed) "shares: the other name sees the change" else "copy:   the other name is untouched"))
}

fun main() {
    var n = 1
    val m = n
    n = 99
    report("Int", m == 99)

    val p = Point(1)
    val q = p
    q.x = 99
    report("object", p.x == 99)

    val list = mutableListOf(1, 2)
    val list2 = list
    list2[0] = 99
    report("List", list[0] == 99)

    // copy() makes a new Point
    val c = p.copy()
    c.x = 5
    println("p.x after changing the copy(): ${p.x}")

    // Identity and equality are different operators
    val r = Point(99)
    println("p === q: ${p === q}  (same object)")
    println("p === r: ${p === r}, p == r: ${p == r}  (same x, different object)")

    // copy() is shallow: the new Team holds the same member list
    val a = Team("red", mutableListOf("ana"))
    val b = a.copy()
    b.members[0] = "zed"
    println("a.members: ${a.members}")

    // A lambda captures the variable itself, so it can change it
    var count = 0
    val inc = { count++ }
    inc()
    inc()
    println("count after two inc() calls: $count")

    // A loop variable is a val: you can change the object it names, not the name
    val people = listOf(Point(1), Point(2))
    for (pt in people) {
        pt.x++
    }
    println("people after for (pt in people) pt.x++: $people")
}
