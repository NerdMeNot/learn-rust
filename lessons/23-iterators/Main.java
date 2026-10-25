import java.util.List;

public class Main {
    public static void main(String[] args) {
        var nums = List.of(1, 2, 3, 4, 5, 6);

        List<Integer> evensSquared = nums.stream()
            .filter(n -> n % 2 == 0)
            .map(n -> n * n)
            .toList();

        int total = nums.stream().mapToInt(Integer::intValue).sum();
        System.out.println(evensSquared + " " + total);
    }
}
