fun makeGreeting(name: String): String = "hello $name"

fun firstWord(text: String): String = text.substringBefore(' ')

fun main() {
    val g = makeGreeting("rust")
    val w = firstWord(g)
    println("$g / $w")
}
