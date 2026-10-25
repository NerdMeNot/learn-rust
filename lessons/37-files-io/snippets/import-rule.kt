import java.nio.file.Files
import java.nio.file.Path
import kotlin.io.path.readText // without this line the call below does not compile

fun main() {
    val path = Path.of(System.getProperty("java.io.tmpdir"), "lr-kt-import.txt")
    Files.writeString(path, "hi")
    println(path.readText())
    Files.delete(path)
}
