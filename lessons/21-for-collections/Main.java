import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        var names = List.of("ana", "bo");
        for (var n : names) {
            System.out.println("hi " + n);
        }

        var nums = new ArrayList<>(List.of(1, 2, 3));
        for (int i = 0; i < nums.size(); i++) {   // change items by index
            nums.set(i, nums.get(i) * 10);
        }

        for (int i = 0; i < nums.size(); i++) {   // with index
            System.out.println(i + ": " + nums.get(i));
        }
        System.out.println(names.size() + " names, " + nums);
    }
}
