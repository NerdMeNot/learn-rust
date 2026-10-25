import java.util.ArrayList;
import java.util.List;

public class Erasure {
    public static void main(String[] args) {
        List<String> words = new ArrayList<>();
        List<Integer> numbers = new ArrayList<>();
        numbers.add(1000);
        numbers.add(1000);
        System.out.println(words.getClass() == numbers.getClass());
        System.out.println(words.getClass().getName());
        System.out.println(numbers.get(0) == numbers.get(1));
    }
}
