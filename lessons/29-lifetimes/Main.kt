// Nothing to annotate: the JVM keeps a String alive while any reference to it exists.
fun longest(a: String, b: String): String = if (a.length >= b.length) a else b

class Excerpt(val part: String)

fun main() {
    val novel = "Call me Ishmael. Some years ago..."
    val first = novel.substringBefore('.')
    val excerpt = Excerpt(first)
    val l = longest(excerpt.part, "short")
    println("longest: $l")
    println("excerpt length: ${excerpt.part.length}")
}
