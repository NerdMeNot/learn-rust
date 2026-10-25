import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

public class Main {
    static String get(int[] a, int i) {
        return i < a.length ? "Some(" + a[i] + ")" : "None";
    }

    public static void main(String[] args) {
        int[] prices = {10, 20, 30};
        // a MemorySegment is a bounds-checked view of memory, read by byte offset
        MemorySegment seg = MemorySegment.ofArray(prices);
        int second = seg.get(ValueLayout.JAVA_INT, 4);
        System.out.println("second, read by offset: " + second);
        System.out.println("get(1): " + get(prices, 1));
        System.out.println("get(9): " + get(prices, 9));
    }
}
