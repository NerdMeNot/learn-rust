data class User(val name: String, val tags: MutableList<String>)

fun main() {
    val a = User("ana", mutableListOf("admin"))
    val b = a.copy(name = "bo")
    b.tags.add("guest")
    println(a)
    println(b)
    println(a == User("ana", mutableListOf("admin", "guest")))
}
