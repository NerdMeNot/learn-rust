fun main() {
    println("0 until 4: " + (0 until 4).toList())
    println("0..4: " + (0..4).toList())
    println("10 downTo 7: " + (10 downTo 7).toList())
    println("9 downTo 1 step 3: " + (9 downTo 1 step 3).toList())
    println("0 until 10 step 3: " + (0 until 10 step 3).toList())
    var count = 0
    repeat(3) { count++ }
    println("repeat(3): $count")
}
