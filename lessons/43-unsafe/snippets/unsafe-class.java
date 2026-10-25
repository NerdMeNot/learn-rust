import java.lang.reflect.Field;
import sun.misc.Unsafe;

public class Main {
    public static void main(String[] args) throws Exception {
        Field f = Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        Unsafe unsafe = (Unsafe) f.get(null);
        long addr = unsafe.allocateMemory(12);
        try {
            unsafe.putInt(addr, 10);
            unsafe.putInt(addr + 4, 20);
            unsafe.putInt(addr + 8, 30);
            System.out.println("second, read by address: " + unsafe.getInt(addr + 4));
        } finally {
            unsafe.freeMemory(addr);
        }
    }
}
