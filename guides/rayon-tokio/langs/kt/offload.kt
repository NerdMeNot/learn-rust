import kotlinx.coroutines.*

fun cpuWork(): Long = (0 until 1_000_000).sumOf { (it % 7).toLong() }

fun main() = runBlocking {
    // 1. CPU-bound work called straight from a coroutine, on a loop with one thread.
    val log = mutableListOf<String>()
    val heartbeat = launch {
        for (i in 1..3) {
            log.add("tick $i")
            yield() // let the loop run something else
        }
    }
    yield() // let the heartbeat start
    log.add("sum done, total ${cpuWork()}")
    heartbeat.join()
    println("called directly: ${log.joinToString(", ")}")

    // 2. The same work moved to Dispatchers.Default, a pool of worker threads.
    println("Dispatchers.Default: ${withContext(Dispatchers.Default) { cpuWork() }}")
}
