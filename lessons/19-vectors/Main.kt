fun main() {
    val v = mutableListOf<Int>()
    v.add(10)
    v.add(20)

    val w = mutableListOf(1, 2, 3)
    w.add(4)

    val part = w.subList(1, 3)
    println("$v $w $part len=${w.size}")
}
