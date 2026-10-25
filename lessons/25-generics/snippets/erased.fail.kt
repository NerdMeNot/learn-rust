fun <T> isA(value: Any): Boolean = value is T

fun main() {
    println(isA<Int>(3))
}
