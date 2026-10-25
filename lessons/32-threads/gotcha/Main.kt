import kotlin.concurrent.thread

fun main() {
    val names = listOf("a", "b", "c")
    val t = thread { println(names.size) }
    t.join()
}
