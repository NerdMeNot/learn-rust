import java.util.PriorityQueue

fun main() {
    val text = "the cat saw the dog and the cat ran"

    val counts = text.split(" ").groupingBy { it }.eachCount()
    val top = counts.entries
        .sortedWith(compareBy({ -it.value }, { it.key }))
        .take(3)
        .map { it.key to it.value }
    println("top: $top")

    val byName = sortedMapOf("zoe" to 31, "ana" to 30, "mia" to 28)
    println("sorted: $byName")

    val seen = text.split(" ").toSet()
    println("${seen.size} unique words, has dog: ${"dog" in seen}")

    val queue = ArrayDeque(listOf("a", "b"))
    queue.addLast("c")
    val first = queue.removeFirst()
    println("served $first, waiting $queue")

    val heap = PriorityQueue(listOf(5, 1, 8, 3))
    print("smallest first:")
    while (heap.isNotEmpty()) print(" ${heap.poll()}")
    println()
}
