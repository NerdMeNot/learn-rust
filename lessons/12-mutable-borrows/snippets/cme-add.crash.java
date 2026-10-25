import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>(List.of(1, 2, 3, 4));
        for (int x : list) {
            System.out.println("saw " + x);
            if (x == 2) list.add(99); // changes the list the loop is walking
        }
    }
}
