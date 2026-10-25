import java.util.concurrent.LinkedBlockingQueue
import kotlin.concurrent.thread

fun main() {
    val queue = LinkedBlockingQueue<MutableList<Int>>()
    val receiver = thread {
        val got = queue.take()
        Thread.sleep(10)
        println("receiver sees $got")
    }
    val msg = mutableListOf(1, 2)
    queue.put(msg)
    msg[0] = 99     // sender keeps changing what it already sent
    receiver.join()
}
