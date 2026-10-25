import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>(List.of(1, 2, 3));
        List<Integer> view = Collections.unmodifiableList(list); // read-only through `view`

        try {
            view.add(4);
        } catch (UnsupportedOperationException e) {
            System.out.println("view.add(4): " + e);
        }

        list.add(5); // the other reference can still change it
        System.out.println("view after list.add(5): " + view);

        List<Integer> fixed = List.of(1, 2, 3); // a list that nobody can change
        try {
            fixed.add(4);
        } catch (UnsupportedOperationException e) {
            System.out.println("List.of(...).add(4): " + e);
        }
    }
}
