fun main() {
    val i: Int = 7
    val l: Long = 10
    println("Int + Long: ${i + l}")
    println("Int + Long has type ${(i + l)::class.simpleName}")
    println("Int * Double: ${i * 2.5}")
    val b1: Byte = 100
    val b2: Byte = 100
    println("Byte + Byte: ${b1 + b2} has type ${(b1 + b2)::class.simpleName}")
    println("1e10.toInt(): ${1e10.toInt()}")
    println("NaN.toInt(): ${Double.NaN.toInt()}")
    val u: UByte = 200u
    println("UByte: $u")
    println("Int.MAX_VALUE + 1: ${Int.MAX_VALUE + 1}")
    println("\"😀\".length: ${"😀".length}")
}
