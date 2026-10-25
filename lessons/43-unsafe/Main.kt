import java.lang.foreign.MemorySegment
import java.lang.foreign.ValueLayout

fun get(a: IntArray, i: Int) = a.getOrNull(i)?.let { "Some($it)" } ?: "None"

fun main() {
    val prices = intArrayOf(10, 20, 30)
    // a MemorySegment is a bounds-checked view of memory, read by byte offset
    val second = MemorySegment.ofArray(prices).get(ValueLayout.JAVA_INT, 4)
    println("second, read by offset: $second")
    println("get(1): ${get(prices, 1)}")
    println("get(9): ${get(prices, 9)}")
}
