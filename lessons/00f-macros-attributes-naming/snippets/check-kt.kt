var bad = false

fun same(got: String, want: String) {
    if (got != want) {
        println("got [$got], want [$want]")
        bad = true
    }
}

fun main() {
    same("%5d".format(42), "   42")
    same("%-5d".format(42), "42   ")
    same("%05d".format(42), "00042")
    same("%+d".format(42), "+42")
    same("%.2f".format(3.14159), "3.14")
    same("%8.3f".format(3.14159), "   3.142")
    same("%x".format(255), "ff")
    same("%#x".format(255), "0xff")
    same("%10s".format("Ada"), "       Ada")
    same("%.3s".format("Lovelace"), "Lov")
    same("%d %s".format(42, "Ada"), "42 Ada")
    if (bad) throw AssertionError("a Kotlin format row is wrong")
    println("ok")
}
