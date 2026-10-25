import kotlinx.coroutines.*

suspend fun fetch(id: Int): String {
    println("fetch $id starts")
    return "user$id"
}

fun main() = runBlocking {
    val first = async { fetch(1) }
    println("after async")
    println(first.await())
    async { fetch(2) }
    println("end of block")
}
