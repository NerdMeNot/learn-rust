fun main() {
    val items = mutableListOf(1, 2, 3)
    val view: List<Int> = items // a List has no add(): read-only through `view`

    items.add(4) // the other reference can still change it
    println("view after items.add(4): $view")

    (view as MutableList<Int>).add(5) // a cast gets the write access back
    println("view after the cast:     $view")

    try {
        for (x in items) {
            if (x == 2) items.add(99)
        }
    } catch (e: ConcurrentModificationException) {
        println("add in a for loop: ${e::class.simpleName}")
    }
}
