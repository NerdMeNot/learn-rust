import kotlinx.coroutines.*

suspend fun fetch(name: String, ms: Long): String {
    delay(ms)
    return "$name done after ${ms}ms"
}

fun main() = runBlocking {
    val users = async { fetch("users", 30) }
    val orders = async { fetch("orders", 10) }
    println(users.await())
    println(orders.await())

    val (left, right) = listOf(async { fetch("left", 20) }, async { fetch("right", 20) }).awaitAll()
    println("$left / $right")

    println(withTimeoutOrNull(50) { fetch("slow", 500) } ?: "slow call timed out")
}
