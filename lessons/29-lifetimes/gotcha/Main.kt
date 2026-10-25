fun longest(a: String, b: String): String = if (a.length >= b.length) a else b

// join builds a new String at run time: a heap object, not a literal.
fun join(vararg parts: String): String = parts.joinToString(" ")

fun main() {
    val s1 = join("a", "long", "first", "string")
    var result = ""
    var other = ""
    run {
        val s2 = join("short", "string")
        result = longest(s1, s2)
        val s3 = join("an", "even", "longer", "second", "string")
        other = longest(s1, s3)
    }
    println("longest: $result")
    println("with a longer s3: $other")
}
