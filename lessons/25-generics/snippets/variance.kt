open class Animal

class Dog : Animal()

fun main() {
    val dogs: List<Dog> = listOf(Dog())
    val animals: List<Animal> = dogs // fine: List<out E> only gives items out
    println("read-only list of dogs used as animals: ${animals.size}")
}
