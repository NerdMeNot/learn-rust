import kotlinx.coroutines.*

suspend fun greet(): String {
    println("greet body runs")
    return "hi"
}

fun main() = runBlocking {
    println("before greet()")
    greet() // a suspend call: the body runs right here
    println("after greet()")

    val job = launch { println("launch body runs") }
    println("after launch: body has run = ${job.isCompleted}")

    val answer = async {
        println("async body runs")
        42
    }
    println("after async: body has run = ${answer.isCompleted}")

    yield() // the first suspension point: main steps aside
    println("after yield")
    println("await: ${answer.await()}")
}
