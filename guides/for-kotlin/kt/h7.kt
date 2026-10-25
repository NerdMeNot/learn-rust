fun Int.speak(): String = "number $this"

fun String.shout(): String = uppercase() + "!"

fun main() {
    println(21.speak())
    println("hi".shout())
}
