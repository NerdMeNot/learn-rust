fun main() {
    val m = Regex("""\d{4}-\d{2}-\d{2}""").find("shipped on 2026-10-25")
    val total = (1..10).toList().parallelStream().mapToInt { it }.sum()
    println("Kotlin: Regex ${m?.value}, parallelStream total $total")
}
