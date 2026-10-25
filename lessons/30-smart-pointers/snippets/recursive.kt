// a class property holds a reference, so a class can hold another of itself
class Expr(val value: Int, val left: Expr?, val right: Expr?)

fun main() {
    val tree = Expr(1, Expr(2, null, null), Expr(3, null, null))
    println("left child: ${tree.left?.value}")
}
