import java.io.File

// Kotlin has no checked exceptions: this compiles with no try and no throws.
fun main() {
    println(File("missing.txt").readText())
}
