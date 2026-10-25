import java.util.ArrayList;
import java.util.List;

public class Main {
    @SuppressWarnings({"rawtypes", "unchecked"})
    public static void main(String[] args) {
        List<String> strings = new ArrayList<>();
        List<Integer> ints = new ArrayList<>();
        System.out.println("class of strings: " + strings.getClass().getName());
        System.out.println("class of ints: " + ints.getClass().getName());
        System.out.println("same class at run time: " + (strings.getClass() == ints.getClass()));

        // The type argument is gone at run time, so a raw list accepts anything.
        List raw = strings;
        raw.add(42);
        System.out.println("strings now holds: " + strings);
        try {
            String first = strings.get(0);
            System.out.println(first);
        } catch (ClassCastException e) {
            System.out.println("reading it as a String fails: " + e.getMessage());
        }
    }
}
