fun main() {
    val day = 6
    val kind = when (day) {
        in 1..5 -> "weekday"
        6, 7 -> "weekend"
        else -> "not a day"
    }
    println(kind)
}
