interface Speaker {
    fun speak(): String
}

// An extension function adds speak() to String...
fun String.speak(): String = "the text $this"

fun introduce(s: Speaker) = println(s.speak())

fun main() {
    println("hi".speak())
    introduce("hi") // ...but String is still not a Speaker
}
