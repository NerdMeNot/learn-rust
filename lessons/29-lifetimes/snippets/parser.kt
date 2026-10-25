class Parser(private var rest: String) {
    fun nextWord(): String? {
        val trimmed = rest.trimStart()
        if (trimmed.isEmpty()) return null
        val space = trimmed.indexOf(' ')
        rest = if (space < 0) "" else trimmed.substring(space + 1)
        return if (space < 0) trimmed else trimmed.substring(0, space)
    }
}

fun main() {
    val parser = Parser("lifetimes are regions")
    val a = parser.nextWord()
    val b = parser.nextWord() // a is still usable: no annotation needed
    println("a: $a")
    println("b: $b")
    val again = Parser("lifetimes are regions").nextWord()
    println("same object as a: ${a === again}")
}
