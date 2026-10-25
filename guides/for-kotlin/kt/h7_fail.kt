interface Speaker {
    fun speak(): String
}

fun Int.speak(): String = "number $this"

fun announce(s: Speaker) = println(s.speak())

fun main() {
    announce(21)
}
