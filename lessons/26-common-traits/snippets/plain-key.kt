class Plain(val major: Int, val minor: Int)

data class Version(val major: Int, val minor: Int)

fun main() {
    val plain = hashMapOf(Plain(1, 2) to "stable")
    println("plain class: ${plain[Plain(1, 2)]}")

    val data = hashMapOf(Version(1, 2) to "stable")
    println("data class: ${data[Version(1, 2)]}")
}
