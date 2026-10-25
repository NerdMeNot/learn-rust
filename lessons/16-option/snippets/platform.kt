fun main() {
    // Java's System.getProperty has no null information, so Kotlin sees a platform type.
    val asNullable: String? = System.getProperty("no.such.property")
    println("as String?: $asNullable")

    try {
        val asString: String = System.getProperty("no.such.property")
        println("not reached: $asString")
    } catch (e: Exception) {
        println("as String: ${e::class.simpleName}")
    }

    val value: String? = null
    try {
        println(value!!.length)
    } catch (e: Exception) {
        println("value!!.length: ${e::class.simpleName}")
    }
}
