class User(val name: String)

fun findUser(id: Int): User? =
    if (id == 1) User("ana") else null

fun main() {
    val a = findUser(1)
    if (a != null) println("found ${a.name}") else println("no user")

    val b = findUser(2)
    println(if (b != null) "found ${b.name}" else "no user")
}
