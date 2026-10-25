import java.util.stream.LongStream

fun main() {
    val total = LongArray(1) // shared by every worker thread
    LongStream.rangeClosed(1, 1_000_000).parallel().forEach { n -> total[0] += n }
    println("total: ${total[0]} expected: 500000500000")
}
