class Address(val city: String?)
class User(val address: Address?)

fun cityLen(u: User?): Int? = u?.address?.city?.length

fun main() {
    val ana = User(Address("Lyon"))
    val bo = User(Address(null))
    println(cityLen(ana))
    println(cityLen(bo))
    println(cityLen(null))
    println(cityLen(null) ?: -1)
    val m: Map<String, String?> = mapOf("k" to null)
    println("${m["k"]} ${m["z"]}")
    println("${"k" in m} ${"z" in m}")
}
