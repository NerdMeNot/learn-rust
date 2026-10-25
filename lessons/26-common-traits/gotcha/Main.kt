data class Version(val major: Int, val minor: Int)

fun main() {
    val notes = hashMapOf<Version, String>()
    notes[Version(1, 2)] = "stable"
    println("found: ${notes[Version(1, 2)]}")
}
