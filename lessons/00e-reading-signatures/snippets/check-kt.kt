import java.io.File

fun <T : Comparable<T>> largest(list: List<T>): T {
    var best = list[0]
    for (item in list) {
        if (item > best) best = item
    }
    return best
}

fun apply(f: (Int) -> Int, x: Int): Int = f(x)

fun find(m: Map<String, Int>, k: String): Int? = m[k]

fun read(path: String): Result<String> = runCatching { File(path).readText() }

class Log {
    private val lines = mutableListOf<String>()
    fun add(line: String) {
        lines.add(line)
    }
    fun count(): Int = lines.size
}

fun main() {
    check(largest(listOf(3, 9, 4)) == 9)
    check(apply({ it + 1 }, 1) == 2)
    check(find(mapOf("a" to 1), "b") == null)
    check(read("/nope").isFailure)
    // a failure in Kotlin's Result is a Throwable
    check(read("/nope").exceptionOrNull() is Throwable)
    val log = Log()
    log.add("hello")
    check(log.count() == 1)
    println("int bits: ${Int.SIZE_BITS}")
    println("kotlin rows ok")
}
