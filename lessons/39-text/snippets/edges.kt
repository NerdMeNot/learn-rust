import java.util.Locale

fun main() {
    println("\"a,b,,\".split(\",\"): " + "a,b,,".split(","))
    println("\"\".split(\",\").size:  " + "".split(",").size)
    println("split(\",\", limit = 2): " + "k,v,w".split(",", limit = 2))
    println("\"a.b\".split(\".\"):    " + "a.b".split("."))

    for (text in listOf("42", "+42", " 42", "1_000")) {
        println("toIntOrNull(\"$text\"): " + text.toIntOrNull())
    }

    println("\"٣\".toIntOrNull(): " + "٣".toIntOrNull())
    println("\"a\\rb\\r\\nc\\n\".lines(): " + "a\rb\r\nc\n".lines())

    Locale.setDefault(Locale.forLanguageTag("tr"))
    println("\"TITLE\".lowercase() under a Turkish default locale: " + "TITLE".lowercase())
}
