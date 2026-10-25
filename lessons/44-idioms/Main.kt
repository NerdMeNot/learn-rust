// The constructor is private, so Port.parse is the only way in.
class Port private constructor(val number: Int) {
    companion object {
        fun parse(text: String): Port {
            val n = text.toLongOrNull() ?: throw IllegalArgumentException("not a number")
            if (n < 1 || n > 65535) throw IllegalArgumentException("$n is outside 1-65535")
            return Port(n.toInt())
        }
    }
}

fun listen(port: Port) {
    println("listening on ${port.number}")
}

fun main() {
    for (text in listOf("8080", "0", "http")) {
        try {
            listen(Port.parse(text))
        } catch (e: IllegalArgumentException) {
            println("\"$text\": ${e.message}")
        }
    }
}
