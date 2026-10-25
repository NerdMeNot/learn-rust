data class Config(val name: String, val port: Int, val verbose: Boolean)

fun main() {
    val base = Config("service", 8080, false)
    val test = base.copy(port = 9000)
    println(base)
    println(test)
}
