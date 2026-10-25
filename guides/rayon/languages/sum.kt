import java.util.stream.LongStream

fun main() {
    val total = LongStream.rangeClosed(1, 1_000_000).parallel().map { n -> n * n }.sum()
    println(total)
}
