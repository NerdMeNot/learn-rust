import java.io.File
import java.nio.file.Files

val root: File = Files.createTempDirectory("ktvis").toFile()

fun write(path: String, text: String) {
    val f = File(root, path)
    f.parentFile.mkdirs()
    f.writeText(text)
}

// Compiles the files with kotlinc and prints the error lines, or "ok".
fun compile(label: String, vararg args: String) {
    println(label)
    val p = ProcessBuilder("kotlinc", *args)
        .directory(root).redirectErrorStream(true).start()
    val output = p.inputStream.bufferedReader().readText()
    p.waitFor()
    val errors = output.lines().filter { ": error: " in it }
    if (errors.isEmpty()) println("  ok") else errors.forEach { println("  $it") }
}

fun main() {
    write("A.kt", """
        package shop
        private fun fileSecret() = 1
        internal fun moduleSecret() = 2
        fun open() = 3
    """.trimIndent())
    write("B.kt", """
        package shop
        fun ok() = open()
        fun bad() = fileSecret()
    """.trimIndent())
    compile("1. another file of the same package calls a private top-level function", "A.kt", "B.kt", "-d", "one")

    write("C.kt", """
        package shop
        class Outer {
            class Nested { private fun hidden() = 4 }
            fun peek() = Nested().hidden()
        }
    """.trimIndent())
    compile("2. an outer class calls a private member of its nested class", "C.kt", "-d", "two")

    write("lib/A.kt", File(root, "A.kt").readText())
    compile("3. build the package as a library", "lib/A.kt", "-d", "lib.jar")
    write("D.kt", """
        package app
        fun a() = shop.open()
        fun b() = shop.moduleSecret()
    """.trimIndent())
    compile("4. another module calls the public and the internal function", "D.kt", "-cp", "lib.jar", "-d", "three")
}
