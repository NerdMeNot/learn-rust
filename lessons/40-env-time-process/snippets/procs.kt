import kotlin.system.exitProcess
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds
import kotlin.time.TimeSource

fun main(args: Array<String>) {
    println("args: ${args.size} (no program name in the array)")

    // a monotonic mark, like Rust's Instant
    val mark = TimeSource.Monotonic.markNow()
    Thread.sleep(20)
    println("elapsed >= 20ms: ${mark.elapsedNow() >= 20.milliseconds}")

    // a Duration is signed
    println("1s - 2s = ${1.seconds - 2.seconds}")

    // the wall clock, as milliseconds since 1970
    println("currentTimeMillis > 0: ${System.currentTimeMillis() > 0}")

    // ?: is the usual way to give a missing variable a default
    println("port: ${System.getenv("RBC_MISSING") ?: "8080"}")

    // the process's own environment is read-only; a child's is edited on the builder
    val env = ProcessBuilder("printenv", "RBC_MODE")
    env.environment()["RBC_MODE"] = "fast"
    println("child sees ${env.start().inputStream.bufferedReader().readText().trim()}")

    // the same ProcessBuilder as Java: no shell, and a failing program is just a code
    val p = ProcessBuilder("false").start()
    println("false exits with ${p.waitFor()}")

    try {
        println("calling exitProcess")
        exitProcess(0)
    } finally {
        println("finally ran")
    }
}
