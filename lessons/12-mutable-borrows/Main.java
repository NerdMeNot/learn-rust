import java.util.ArrayList;
import java.util.List;

public class Main {
    static void addBonus(List<Integer> scores, int bonus) {
        scores.replaceAll(s -> s + bonus);
        scores.add(0);
    }

    public static void main(String[] args) {
        var scores = new ArrayList<>(List.of(10, 20));
        addBonus(scores, 5);
        System.out.println(scores);
    }
}
