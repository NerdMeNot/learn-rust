import java.util.concurrent.atomic.AtomicInteger
import kotlin.concurrent.thread

fun main() {
    var count = 0
    val gate = Any()
    val atomic = AtomicInteger()
    val threads = (1..8).map {
        thread {
            repeat(1000) {
                synchronized(gate) { count++ }
                atomic.incrementAndGet()
            }
        }
    }
    threads.forEach { it.join() }
    println("synchronized: $count")
    println("AtomicInteger: ${atomic.get()}")
}
