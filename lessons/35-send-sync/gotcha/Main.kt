import kotlin.concurrent.thread

class Job(val name: String, val log: MutableList<String>)

fun run(job: Job) {
    job.log.add(job.name)
}

fun main() {
    val log = mutableListOf<String>()
    val job = Job("resize", log)
    val t = thread { run(job) }
    t.join()
    println("log: " + log.joinToString(", ", "[", "]") { "\"$it\"" })
}
