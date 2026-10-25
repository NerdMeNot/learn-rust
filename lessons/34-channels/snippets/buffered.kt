import kotlinx.coroutines.*
import kotlinx.coroutines.channels.*

fun main() = runBlocking {
    val ch = Channel<Int>(2) // like sync_channel(2)
    println("trySend 1: ${ch.trySend(1).isSuccess}")
    println("trySend 2: ${ch.trySend(2).isSuccess}")
    println("trySend 3: ${ch.trySend(3).isSuccess}")

    ch.close() // the sender side closes
    println("trySend after close: closed = ${ch.trySend(4).isClosed}")
    try {
        ch.send(5)
    } catch (e: ClosedSendChannelException) {
        println("send after close: ${e::class.simpleName}")
    }

    for (x in ch) println("received: $x") // queued items first, then the loop ends
    println("receiveCatching at the end: closed = ${ch.receiveCatching().isClosed}")

    val other = Channel<Int>(2)
    other.cancel() // the receiver side hangs up
    println("trySend after cancel: closed = ${other.trySend(1).isClosed}")
    try {
        other.send(1)
    } catch (e: CancellationException) {
        println("send after cancel: ${e::class.simpleName}")
    }

    val unlimited = Channel<Int>(Channel.UNLIMITED) // like channel()
    val accepted = (1..1000).count { unlimited.trySend(it).isSuccess }
    println("UNLIMITED accepted $accepted of 1000 with no receiver")
}
