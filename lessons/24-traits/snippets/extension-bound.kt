interface Speaker {
    fun speak(): String
}

class Dog : Speaker {
    override fun speak() = "woof"
}

// An extension on a bounded type parameter: every Speaker gets shout().
fun <T : Speaker> T.shout(): String = speak().uppercase()

fun main() {
    println(Dog().shout())
}
