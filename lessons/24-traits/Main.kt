interface Speaker {
    fun speak(): String
    fun shout(): String = speak().uppercase()
}

object Dog : Speaker {
    override fun speak() = "woof"
}

class Robot(val id: Int) : Speaker {
    override fun speak() = "beep $id"
}

fun introduce(s: Speaker) {
    println("${s.speak()} / ${s.shout()}")
}

fun main() {
    introduce(Dog)
    introduce(Robot(7))
}
