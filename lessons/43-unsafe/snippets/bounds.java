import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

public class Main {
    public static void main(String[] args) {
        MemorySegment seg = MemorySegment.ofArray(new int[] {10, 20, 30});
        try {
            seg.get(ValueLayout.JAVA_INT, 12); // byte offset 12 is just past the end
        } catch (IndexOutOfBoundsException e) {
            System.out.println("caught " + e.getClass().getSimpleName());
        }
    }
}
