// `reified` keeps T at run time. Without it, Kotlin erases T like Java does.
inline fun <reified T> isA(value: Any): Boolean = value is T

fun main() {
    println("3 is an Int: ${isA<Int>(3)}")
    println("\"3\" is an Int: ${isA<Int>("3")}")
}
