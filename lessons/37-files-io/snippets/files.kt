import java.io.File

fun main() {
    try {
        File("/no/such/file.txt").readText()
    } catch (e: java.io.IOException) {
        println("${e.javaClass.simpleName}: ${e.message}")
    }

    val f = File(System.getProperty("java.io.tmpdir"), "lr-fio-kt.txt")
    f.writeBytes(byteArrayOf('b'.code.toByte(), 0xff.toByte(), 'x'.code.toByte(), 10))
    val text = f.readText()
    println("readText on invalid UTF-8: no exception, chars: ${text.trimEnd().map { it.code.toString(16) }}")

    // useLines closes the file when the block ends
    val n = f.useLines { lines -> lines.count() }
    println("lines: $n")
    f.delete()

    // close() flushes the buffer, and use { } reports a failure there.
    // /dev/full refuses every byte, like a full disk (Linux).
    try {
        File("/dev/full").bufferedWriter().use { it.write("data") }
    } catch (e: java.io.IOException) {
        println("use { } reported: ${e.message}")
    }
}
