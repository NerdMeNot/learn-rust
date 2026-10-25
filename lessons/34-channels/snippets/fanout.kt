import kotlinx.coroutines.*
import kotlinx.coroutines.channels.*

fun main() = runBlocking {
    val ch = Channel<Int>() // two coroutines receive from one channel
    val first = mutableListOf<Int>()
    val second = mutableListOf<Int>()
    val a = launch { for (x in ch) { first.add(x); yield() } }
    val b = launch { for (x in ch) { second.add(x); yield() } }
    for (i in 1..6) ch.send(i)
    ch.close()
    joinAll(a, b)
    println("first receiver got $first")
    println("second receiver got $second")
}
