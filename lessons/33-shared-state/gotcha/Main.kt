import kotlin.concurrent.thread

fun main() {
    val results = ArrayList<Int>()
    val threads = (0 until 3).map { i ->
        thread { results.add(i * 10) }   // no lock!
    }
    threads.forEach { it.join() }
    println(results.size)
}
