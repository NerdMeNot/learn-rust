// 1. Default and named arguments, and copy(): what a Rust builder or struct update stands in for.
data class Config(val host: String = "localhost", val port: Int = 8080, val retries: Int = 3, val verbose: Boolean = false)

// 2. A value class is a distinct type with the same representation as its single field.
@JvmInline
value class UserId(val value: Int)

fun main() {
    val plain = Config()
    val custom = Config(port = 9000, verbose = true)
    val moved = plain.copy(port = 9001)
    println(plain)
    println(custom)
    println(moved)
    println("UserId(7) = ${UserId(7)}")
}
