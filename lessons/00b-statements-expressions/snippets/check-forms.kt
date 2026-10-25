fun log(msg: String) {
    println("log: $msg")
}

fun addOne(x: Int) = x + 1

fun main() {
    val n = 7

    val size = if (n > 5) "big" else "small"
    println("ternary: $size")

    val area = run {
        val w = 3
        val h = 4
        w * h
    }
    println("block: $area")

    val name = when (2) {
        1 -> "one"
        2 -> "two"
        else -> "many"
    }
    println("switch: $name")

    var a = 1
    var b = 2
    a = b.also { b = a }
    println("swap: $a $b")

    val c = run {
        log("first")
        2
    }
    println("comma: $c")

    val closure = { x: Int ->
        val y = x + 1
        y
    }
    println("closure: ${closure(41)} ${addOne(41)}")
}
