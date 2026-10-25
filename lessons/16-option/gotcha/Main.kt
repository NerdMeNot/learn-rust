class User(val name: String)

fun findUser(id: Int): User? =
    if (id == 1) User("ana") else null

fun main() {
    val u = findUser(2)
    println(u.name)
}
