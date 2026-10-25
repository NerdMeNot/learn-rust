import kotlin.coroutines.Continuation
import kotlin.coroutines.EmptyCoroutineContext
import kotlin.coroutines.createCoroutine
import kotlin.coroutines.resume

fun main() {
    println("start")
    // createCoroutine builds the coroutine without starting it
    val coroutine = suspend { println("task body runs") }
        .createCoroutine(Continuation(EmptyCoroutineContext) { })
    println("task created")
    coroutine.resume(Unit)
    println("end")
}
