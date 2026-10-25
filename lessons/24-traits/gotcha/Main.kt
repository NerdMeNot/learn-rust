interface Speaker {
    fun speak(): String
}

class Robot(val id: Int) { // never says ": Speaker"
    fun speak(): String = "beep $id"
}

fun introduce(s: Speaker) = println(s.speak())

fun main() {
    introduce(Robot(7))
}
