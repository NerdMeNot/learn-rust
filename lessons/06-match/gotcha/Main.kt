fun main() {
    val day = 6
    var kind = ""
    when (day) {
        in 1..5 -> kind = "weekday"
        6, 7 -> kind = "weekend"
    }
    println(kind)
}
