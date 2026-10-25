import kotlin.concurrent.thread

fun main() {
    // kotlin.concurrent.thread starts a java.lang.Thread
    val t = thread(name = "worker-1") {
        throw IllegalStateException("worker failed")
    }
    t.join()
    println("main: join returned normally")
}
