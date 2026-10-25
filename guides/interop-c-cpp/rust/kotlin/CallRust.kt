import java.lang.foreign.Arena
import java.lang.foreign.FunctionDescriptor
import java.lang.foreign.Linker
import java.lang.foreign.MemorySegment
import java.lang.foreign.SymbolLookup
import java.lang.foreign.ValueLayout
import java.nio.file.Path

fun main(args: Array<String>) {
    val linker = Linker.nativeLinker()
    Arena.ofConfined().use { arena ->
        val rust = SymbolLookup.libraryLookup(Path.of(args[0]), arena)
        val greeting = linker.downcallHandle(
            rust.find("greeting").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS))
        val greetingFree = linker.downcallHandle(
            rust.find("greeting_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS))
        val text = greeting.invokeWithArguments(arena.allocateFrom("Kotlin")) as MemorySegment
        println(text.reinterpret(Long.MAX_VALUE).getString(0))
        greetingFree.invokeWithArguments(text)
    }
}
