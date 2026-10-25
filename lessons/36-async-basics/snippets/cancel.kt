import kotlinx.coroutines.*

fun main() = runBlocking {
    val job = launch {
        cancel() // ask this coroutine to stop
        println("after cancel(): isActive = $isActive, and the body is still running")
        try {
            yield() // a suspension point checks for cancellation
        } catch (e: CancellationException) {
            println("caught CancellationException: ${e.message}")
            throw e
        }
        println("not printed")
    }
    job.join()
    println("isCancelled = ${job.isCancelled}")
}
