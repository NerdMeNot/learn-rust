fun main() {
    val owned = "héllo"
    val view = owned.substring(0, 1)

    val s = StringBuilder("go")
    s.append(" + rust")

    println("$owned $view $s")
    println("bytes=${owned.toByteArray().size} chars=${owned.length}")
    for (c in owned) {
        print("[$c]")
    }
    println()
}
