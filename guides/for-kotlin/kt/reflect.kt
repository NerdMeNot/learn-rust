import kotlin.reflect.full.memberProperties

data class Pt(val x: Int, val y: Int)

fun main() {
    println(Pt::class.memberProperties.map { it.name }.sorted())
}
