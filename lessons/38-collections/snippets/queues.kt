import java.util.PriorityQueue
import java.util.TreeMap

fun main() {
    val q = ArrayDeque<String>()
    q.addLast("b")
    q.addFirst("a")
    println("ArrayDeque: $q, class: ${q.javaClass.name}")
    println("removeFirst: ${q.removeFirst()}")

    val log = sortedMapOf(10 to "boot", 25 to "login", 40 to "upload")
    println("sortedMapOf: $log, runtime class: ${log.javaClass.name}")
    // The static type of `log` is SortedMap, which has no floorKey: build a TreeMap for that.
    val tree = TreeMap(log)
    println("TreeMap floorKey(30): ${tree.floorKey(30)}, ceilingKey(30): ${tree.ceilingKey(30)}")

    println("setOf is a ${setOf(3, 1, 2).javaClass.name}")
    println("mutableMapOf is a ${mutableMapOf(3 to 1).javaClass.name}")
    println("hashSetOf is a ${hashSetOf(3, 1, 2).javaClass.name}")

    val a = setOf(1, 2, 3, 4)
    val b = setOf(3, 4, 5)
    println("a intersect b: ${a intersect b}, a is still: $a")

    val pq = PriorityQueue(listOf(3, 9, 4, 1))
    println("PriorityQueue peek: ${pq.peek()}")
}
