open class Animal

class Dog : Animal()

fun main() {
    val dogs: MutableList<Dog> = mutableListOf(Dog())
    val animals: MutableList<Animal> = dogs // could add a Cat to it
    println(animals.size)
}
