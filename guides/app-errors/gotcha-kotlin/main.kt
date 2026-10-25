import java.io.File
import java.io.IOException

// Nothing in the signature says this can throw, or what.
fun readConfig(path: String): String = File(path).readText()

fun main() {
    try {
        println(readConfig("missing.conf"))
    } catch (e: IOException) {
        println("caught ${e.javaClass.simpleName}")
    }
}
