import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>(List.of(10, 20, 30));
        int[] array = {10, 20, 30};
        try {
            list.get(10);
        } catch (Exception e) {
            System.out.println("ArrayList.get(10): " + e);
        }
        try {
            System.out.println(array[10]);
        } catch (Exception e) {
            System.out.println("int[10]: " + e);
        }
    }
}
