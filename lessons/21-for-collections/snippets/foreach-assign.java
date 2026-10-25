import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> nums = new ArrayList<>(List.of(1, 2, 3));
        for (int n : nums) {
            n *= 10; // compiles: n is a local copy
        }
        System.out.println("after n *= 10: " + nums);

        List<StringBuilder> texts = List.of(new StringBuilder("a"), new StringBuilder("b"));
        for (StringBuilder t : texts) {
            t.append("!"); // changes the object the list refers to
        }
        System.out.println("after t.append: " + texts);
    }
}
