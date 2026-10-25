// Number has no `plus`, so there is no way to say "any type with +".
fun <T : Number> add(a: T, b: T): T = a + b

fun main() {
    println(add(2, 3))
    println(add(1.5, 2.25))
}
