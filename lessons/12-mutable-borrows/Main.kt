fun addBonus(scores: MutableList<Int>, bonus: Int) {
    for (i in scores.indices) {
        scores[i] += bonus
    }
    scores.add(0)
}

fun main() {
    val scores = mutableListOf(10, 20)
    addBonus(scores, 5)
    println(scores)
}
