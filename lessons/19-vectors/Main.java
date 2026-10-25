import java.util.ArrayList;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> v = new ArrayList<>();
        v.add(10);
        v.add(20);

        var w = new ArrayList<>(List.of(1, 2, 3));
        w.add(4);

        List<Integer> part = w.subList(1, 3);   // a view into w, no copy
        System.out.println(v + " " + w + " " + part + " len=" + w.size());
    }
}
