fun main() {
    for (i in 1 until 4) {
        println("for $i")
    }

    var n = 3
    while (n > 0) {
        n--
    }

    var tries = 0
    var found = 0
    while (true) {
        tries++
        if (tries == 3) {
            found = tries * 10
            break
        }
    }
    println("n=$n found=$found")
}
