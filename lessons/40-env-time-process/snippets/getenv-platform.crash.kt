fun main() {
    // System.getenv is a Java method, so Kotlin sees `String!`: it may be null, and Kotlin does not check
    val port = System.getenv("RBC_PORT")
    println("listening on port " + port)
    println("length: " + port.length)
}
