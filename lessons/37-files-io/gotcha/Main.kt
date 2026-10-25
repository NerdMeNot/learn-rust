import java.io.File

fun main() {
    val file = File(System.getProperty("java.io.tmpdir"), "rbc-hello.txt")
    file.writeText("hello")

    val text = file.reader().use { it.readText() }
    println(text)
    file.delete()
}
