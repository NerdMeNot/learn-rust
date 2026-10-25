fun main(args: Array<String>) {
    println("${args.size} arg(s) after the program name")

    val port = System.getenv("RBC_PORT") ?: "8080"
    println("port $port")
    val missing = System.getenv("RBC_MISSING")
    println(if (missing != null) "RBC_MISSING=$missing" else "RBC_MISSING: not set")

    val start = System.nanoTime()
    Thread.sleep(20)
    val tookMs = (System.nanoTime() - start) / 1_000_000
    println("slept at least 20ms: ${tookMs >= 20}")

    val p = ProcessBuilder("echo", "hello from echo").start()
    print(p.inputStream.bufferedReader().readText())
    println("exit ok: ${p.waitFor() == 0}")
}
