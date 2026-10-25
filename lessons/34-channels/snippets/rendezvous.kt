import kotlinx.coroutines.*
import kotlinx.coroutines.channels.*

fun main() = runBlocking {
    val ch = Channel<Int>() // RENDEZVOUS, capacity 0
    println("trySend with no receiver: ${ch.trySend(1).isSuccess}")

    val sender = launch {
        println("sender: send(2)")
        ch.send(2)
        println("sender: send(2) returned")
    }
    yield() // let the sender run: it suspends in send
    println("main: still running, sender done = ${sender.isCompleted}")
    println("main: receive = ${ch.receive()}")
    sender.join()
}
