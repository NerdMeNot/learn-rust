fun clamp(x: Int): Int {
    if (x < 0) {
        return 0
    }
    return if (x > 100) 100 else x
}

fun main() {
    val n = 7
    val label = if (n % 2 == 0) "even" else "odd"
    val area = run {
        val w = 3
        val h = 4
        w * h
    }
    println("$n is $label")
    println("area: $area")
    println("clamp: ${clamp(-5)} ${clamp(50)} ${clamp(250)}")
}
