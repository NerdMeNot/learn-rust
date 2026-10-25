import java.util.*;

public class Main {
    // returns a piece of the input: a view of it, not a copy
    static List<Integer> firstTwo(List<Integer> xs) {
        return xs.subList(0, 2);
    }

    public static void main(String[] args) {
        List<Integer> scores = new ArrayList<>(List.of(3, 9, 4));
        List<Integer> top = firstTwo(scores);
        System.out.println("top: " + top);
        scores.add(7); // the owner changes the list while the view exists
        System.out.println("top: " + top);
    }
}
