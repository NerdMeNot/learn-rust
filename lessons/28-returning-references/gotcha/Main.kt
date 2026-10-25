fun makeGreeting(name: String): String {
    val s = "hello $name"
    return s
}

fun main() {
    println(makeGreeting("rust"))
}
