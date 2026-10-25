interface Speaker {
    fun speak(): String

    fun shout(): String = speak().uppercase()
}

class Parrot : Speaker {
    override fun speak() = "hello"

    override fun shout() = super<Speaker>.shout() + "!!" // the default this method replaces
}

fun main() {
    println(Parrot().shout())
}
