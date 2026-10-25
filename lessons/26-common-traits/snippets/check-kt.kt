data class Money(val cents: Int) {
    operator fun plus(other: Money) = Money(cents + other.cents)
}

fun main() {
    check("4x".toIntOrNull() == null)
    check("42".toIntOrNull() == 42)
    check(Money(150) + Money(250) == Money(400))
    check(Money(1) == Money(1) && Money(1).hashCode() == Money(1).hashCode())
    check(Money(1).copy(cents = 5) == Money(5)) // copy(...) is struct update
    check((Money(1) as Any) !is Comparable<*>) // a data class is not Comparable
    println("all checks passed")
}
