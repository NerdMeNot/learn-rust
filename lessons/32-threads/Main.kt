import kotlin.concurrent.thread

fun main() {
    val names = listOf("a", "b", "c")
    var count = 0

    val t = thread {
        for (n in names) {
            println("worker sees $n")
        }
        count = names.size
    }

    t.join()
    println("worker counted $count")
}
