class Account(val owner: String, val balance: Int = 0, val currency: String = "USD")

fun main() {
    val a = Account("ana", currency = "EUR")
    println("${a.owner} ${a.balance} ${a.currency}")
}
