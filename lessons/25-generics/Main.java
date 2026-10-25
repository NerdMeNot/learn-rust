import java.util.List;

public class Main {
    // T can be any type that can be compared to itself
    static <T extends Comparable<T>> T largest(List<T> items) {
        T best = items.get(0);
        for (T item : items) {
            if (item.compareTo(best) > 0) {
                best = item;
            }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(largest(List.of(3, 9, 2)));
        System.out.println(largest(List.of("kiwi", "apple", "pear")));
    }
}
