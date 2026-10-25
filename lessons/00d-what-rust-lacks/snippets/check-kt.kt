fun area(width: Int, height: Int = 1) = width * height

fun hello(name: String, greeting: String = "Hello") = "$greeting, $name!"

open class Animal(val name: String)

class Dog(name: String) : Animal(name)

sealed class Shape

class Circle(val r: Int) : Shape()

data class Point(val x: Int, val y: Int)

var visits = 0

fun parse(s: String): Int = s.toInt() + 1

fun firstOrMinus(v: List<Int>): Int {
    val f = v.firstOrNull() ?: return -1
    return f
}

fun main() {
    // ++ and --, also inside expressions
    val arr = intArrayOf(10, 20, 30)
    var i = 0
    val first = arr[i++]
    val second = arr[++i]
    check(first == 10 && second == 30 && i == 2) { "inc" }
    var j = 5
    val pre = --j
    check(pre == 4 && j == 4) { "--x decrements" }

    // if is an expression
    val label = if (i > 1) "big" else "small"
    check(label == "big") { "if" }

    // safe call, Elvis, not-null assertion
    val name: String? = null
    val other: String? = "abc"
    check(name?.length == null && other?.length == 3) { "safe call" }
    check((name ?: "anon") == "anon") { "elvis" }
    check(other!!.length == 3) { "bang bang" }
    check(firstOrMinus(listOf()) == -1 && firstOrMinus(listOf(7)) == 7) { "elvis return" }

    // no new keyword
    val p = Point(1, 2)
    check(p.x == 1) { "no new" }
    check(p.copy(x = 5).y == 2) { "copy" }
    check(Circle(1).r == 1) { "sealed" }

    // default and named arguments
    check(area(3) == 3 && area(width = 3, height = 4) == 12) { "named" }
    check(hello("Ada") == "Hello, Ada!" && hello("Ada", greeting = "Hi") == "Hi, Ada!") { "default" }

    // when: several values in one branch, no fallthrough
    var out = ""
    when (2) {
        1, 2 -> out += "small"
        3 -> out += "three"
        else -> out += "other"
    }
    check(out == "small") { "when" }

    // exceptions
    try {
        parse("x")
        check(false) { "no exception" }
    } catch (e: NumberFormatException) {
        out = "caught"
    }
    check(out == "caught") { "catch" }
    check(parse("41") == 42) { "parse" }

    // inheritance
    val d: Animal = Dog("Rex")
    check(d.name == "Rex") { "inherit" }

    // a global that any function may change
    visits++
    visits++
    check(visits == 2) { "global" }

    // Int to Long is an explicit call
    val big: Long = i.toLong()
    check(big == 2L) { "long" }

    println("kotlin: all checks passed")
}
