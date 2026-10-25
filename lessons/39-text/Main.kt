fun main() {
    val line = "  name=Ana, age=30, city=Lisbon  "

    val clean = line.trim()
    println("[$clean]")

    for (field in clean.split(",")) {
        val parts = field.trim().split("=", limit = 2)
        if (parts.size == 2) println("${parts[0].padStart(5)} -> ${parts[1]}")
    }

    val age = "30".toIntOrNull() ?: 0
    println("next year: ${age + 1}")

    println(clean.uppercase().replace(", ", " | "))

    println("starts with name: ${clean.startsWith("name")}")
    println("position of age: ${clean.indexOf("age")}")

    println(listOf("build", "with", "rust").joinToString("-"))

    val sb = StringBuilder()
    for (i in 1..3) sb.append(i).append(';')
    println(sb)
}
