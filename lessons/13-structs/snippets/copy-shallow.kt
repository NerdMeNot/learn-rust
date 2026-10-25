data class Team(val name: String, val members: MutableList<String>)

fun main() {
    val a = Team("red", mutableListOf("ana"))
    val b = a.copy(name = "blue")
    b.members.add("bob")
    println("a: $a")
    println("b: $b")
    println("same list object: ${a.members === b.members}")
}
