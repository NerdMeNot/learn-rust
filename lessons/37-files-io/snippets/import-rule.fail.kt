import java.nio.file.Files
import java.nio.file.Path

fun main() {
    val path = Path.of(System.getProperty("java.io.tmpdir"), "lr-kt-import.txt")
    Files.writeString(path, "hi")
    println(path.readText()) // readText on a java.nio.file.Path is not imported
    Files.delete(path)
}
