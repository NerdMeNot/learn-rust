import java.io.File
import java.io.FileNotFoundException

fun main() {
    val file = File(System.getProperty("java.io.tmpdir"), "rbc-notes.txt")

    file.writeText("milk\neggs\n")
    file.appendText("bread\n")

    val all = file.readText()
    println("${all.length} chars")

    file.useLines { lines ->
        lines.forEachIndexed { i, line -> println("${i + 1}: $line") }
    }

    println("file name: \"${file.name}\"")

    try {
        File("/no/such/file.txt").readText()
    } catch (e: FileNotFoundException) {
        println("error: ${e.message}")
    }

    file.delete()
}
