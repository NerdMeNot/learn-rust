fun main() {
    val label: String
    if (System.getenv("X") != null) {
        label = "big"
    }
    println(label)
}
