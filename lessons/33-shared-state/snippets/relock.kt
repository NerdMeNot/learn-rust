val lock = Any()

fun inner() {
    synchronized(lock) {
        println("inner: same thread, lock already held, not blocked")
    }
}

fun main() {
    synchronized(lock) {
        inner()
    }
}
