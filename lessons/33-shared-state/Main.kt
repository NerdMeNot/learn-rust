import kotlin.concurrent.thread

fun main() {
    val lock = Any()
    val results = mutableListOf<Int>()

    val threads = (0 until 3).map { i ->
        thread {
            synchronized(lock) {
                results.add(i * 10)
            }
        }
    }
    threads.forEach { it.join() }

    results.sort()
    println(results)
}
