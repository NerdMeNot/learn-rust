import java.util.concurrent.TimeUnit

fun main() {
    // waitFor(timeout, unit): false if the child is still running when the time is up
    val slow = ProcessBuilder("sleep", "60").start()
    println("finished within the limit: ${slow.waitFor(200, TimeUnit.MILLISECONDS)}")
    slow.destroyForcibly()
    println("after destroyForcibly, waitFor() = ${slow.waitFor()}")

    // a child ended by a signal looks like an exit code of 128 + the signal number
    val killed = ProcessBuilder("sh", "-c", "kill -9 \$\$").start()
    val exited = ProcessBuilder("sh", "-c", "exit 137").start()
    println("kill -9: ${killed.waitFor()}, exit 137: ${exited.waitFor()}")
}
