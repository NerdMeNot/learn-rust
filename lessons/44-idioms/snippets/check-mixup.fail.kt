@JvmInline
value class UserId(val value: Int)

@JvmInline
value class OrderId(val value: Int)

fun loadUser(id: UserId): String = "user #${id.value}"

fun main() {
    val order = OrderId(7)
    println(loadUser(order))
}
