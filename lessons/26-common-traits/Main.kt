// A data class writes equals, hashCode and toString from the constructor properties.
data class Version(val major: Int, val minor: Int) : Comparable<Version> {
    // Ordering is not automatic: Comparable is the one to implement.
    override fun compareTo(other: Version): Int =
        compareValuesBy(this, other, Version::major, Version::minor)

    // toString is the user-facing text (a data class's own would be "Version(major=1, minor=10)").
    override fun toString() = "v$major.$minor"
}

fun main() {
    val a = Version(1, 10)
    val b = Version(1, 9)
    println("display: $a")
    println("a == b: ${a == b}")
    println("a > b: ${a > b}")

    val all = listOf(a, b, Version(0, 0)).sorted()
    println("sorted: ${all.joinToString(" ")}")

    val seen = hashSetOf<Version>()
    for (v in listOf(a, b, Version(0, 0), a)) seen.add(v)
    println("distinct: ${seen.size}")
}
