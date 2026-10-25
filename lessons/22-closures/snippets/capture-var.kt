fun main() {
    var x = 1
    val show = { println("x: $x") }
    x = 2
    show()

    val bump = { x += 10 }
    bump()
    println("x after bump: $x")
}
