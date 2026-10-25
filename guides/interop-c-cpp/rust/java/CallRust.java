import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;

public class CallRust {
    public static void main(String[] args) throws Throwable {
        Linker linker = Linker.nativeLinker();
        try (Arena arena = Arena.ofConfined()) {
            SymbolLookup rust = SymbolLookup.libraryLookup(Path.of(args[0]), arena);

            // struct Point { double x; double y; } is passed by value
            StructLayout point = MemoryLayout.structLayout(
                ValueLayout.JAVA_DOUBLE.withName("x"), ValueLayout.JAVA_DOUBLE.withName("y"));
            MethodHandle distance = linker.downcallHandle(
                rust.find("point_distance").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_DOUBLE, point, point));
            MemorySegment a = arena.allocate(point);
            MemorySegment b = arena.allocate(point);
            b.set(ValueLayout.JAVA_DOUBLE, 0, 3.0);
            b.set(ValueLayout.JAVA_DOUBLE, 8, 4.0);
            System.out.println("distance: " + (double) distance.invokeExact(a, b));

            MethodHandle greeting = linker.downcallHandle(
                rust.find("greeting").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
            MethodHandle greetingFree = linker.downcallHandle(
                rust.find("greeting_free").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
            MemorySegment name = arena.allocateFrom("Java");
            MemorySegment text = (MemorySegment) greeting.invokeExact(name);
            // The returned address has size 0 until you say how long the memory is.
            System.out.println(text.reinterpret(Long.MAX_VALUE).getString(0));
            greetingFree.invokeExact(text);
        }
    }
}
