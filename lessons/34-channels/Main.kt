import java.util.concurrent.LinkedBlockingQueue
import kotlin.concurrent.thread

fun main() {
    val queue = LinkedBlockingQueue<String>()

    val worker = thread {
        for (i in 1..3) {
            queue.put("job $i")
        }
        queue.put("DONE")      // no built-in close, so send a marker
    }

    while (true) {
        val msg = queue.take()
        if (msg == "DONE") break
        println("got $msg")
    }
    worker.join()
}
