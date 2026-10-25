fun main() {
    val list: List<Int> = listOf(1000, 1000)
    val first: Any = list[0]
    println("element of a List<Int>: ${first.javaClass.name}")
    println("same object twice: ${(list[0] as Any) === (list[1] as Any)}")
    // IntArray is the unboxed form.
    println("IntArray: ${intArrayOf(1000, 1000).javaClass.name}")
}
