data class User(
    val name: String,
    val age: Int,
    val active: Boolean,
)

fun main() {
    val u = User(name = "ana", age = 30, active = true)
    println("${u.name} is ${u.age}, active=${u.active}")
    println(u)
}
