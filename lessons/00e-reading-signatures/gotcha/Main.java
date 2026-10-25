import java.util.List;

public class Main {
    static <T> T largest(List<T> list) {
        T best = list.get(0);
        for (T item : list) {
            if (item > best) {
                best = item;
            }
        }
        return best;
    }

    public static void main(String[] args) {
        List<Integer> numbers = List.of(3, 9, 4);
        List<String> words = List.of("pear", "fig", "apple");
        System.out.println("largest number: " + largest(numbers));
        System.out.println("largest word: " + largest(words));
    }
}
