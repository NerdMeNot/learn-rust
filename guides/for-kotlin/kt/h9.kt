class Server(val host: String = "localhost", val port: Int = 8080, val tls: Boolean = false)

fun main() {
    val s = Server(port = 9000)
    println("${s.host}:${s.port} tls=${s.tls}")
    val cfg = mutableMapOf<String, String>().apply { put("a", "1") }
    println(cfg)
    val next = "41".toIntOrNull()?.let { it + 1 }
    println(next)
}
