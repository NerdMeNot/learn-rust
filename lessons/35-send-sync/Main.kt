import java.util.concurrent.Executors

fun main() {
    val words = listOf("alpha", "beta")
    val pool = Executors.newFixedThreadPool(2)

    // hand the list to a pool thread and get it back as the result
    val back = pool.submit<List<String>> { words }.get()
    println("moved there and back: " + back.joinToString(", ", "[", "]") { "\"$it\"" })

    // two threads read the same list at once; nothing checks that this is safe
    val a = pool.submit<Int> { words[0].length }
    val b = pool.submit<Int> { words[1].length }
    println("lengths read in parallel: ${a.get()} and ${b.get()}")

    pool.shutdown()
}
