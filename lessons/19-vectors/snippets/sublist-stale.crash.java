import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> v = new ArrayList<>(List.of(1, 2, 3));
        List<Integer> view = v.subList(0, 2); // a live view of v
        v.add(4);                             // changes v's size
        System.out.println(view);             // using the view now throws
    }
}
