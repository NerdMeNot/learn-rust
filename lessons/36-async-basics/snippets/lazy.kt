import kotlinx.coroutines.*

fun main() = runBlocking {
    val job = launch(start = CoroutineStart.LAZY) { println("lazy body runs") }
    println("created: isActive = ${job.isActive}")
    yield()
    println("after yield: isActive = ${job.isActive}")
    job.start()
    println("after start(): isActive = ${job.isActive}")
    job.join()
    println("after join(): isCompleted = ${job.isCompleted}")

    val value = async(start = CoroutineStart.LAZY) {
        println("lazy async body runs")
        7
    }
    println("await starts it: ${value.await()}")
}
