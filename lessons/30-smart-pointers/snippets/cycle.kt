import java.lang.ref.WeakReference

class Node {
    var other: Node? = null
}

fun makeCycle(): WeakReference<Node> {
    val a = Node()
    val b = Node()
    a.other = b
    b.other = a
    return WeakReference(a) // does not keep a alive
}

fun main() {
    val weak = makeCycle()
    var i = 0
    while (i < 20 && weak.get() != null) {
        System.gc() // a request, so ask a few times
        Thread.sleep(50)
        i++
    }
    println("cycle collected: ${weak.get() == null}")
}
