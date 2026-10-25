import kotlin.concurrent.thread

fun main() {
    val data = mutableListOf(1, 2, 3)
    val lock = Any()

    val worker = thread {
        try {
            synchronized(lock) {
                data.add(4)
                throw IllegalStateException("worker failed halfway")
            }
        } catch (e: IllegalStateException) {
            println("worker died: ${e.message}")
        }
    }
    worker.join()

    synchronized(lock) { // not poisoned: leaving the block released the lock
        println("main got the lock; data: $data")
    }
}
