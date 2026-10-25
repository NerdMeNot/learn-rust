import java.util.List;

public class Main {
    static int total(List<Integer> xs) {
        int sum = 0;
        for (int x : xs) {
            sum += x;
        }
        return sum;
    }

    public static void main(String[] args) {
        var nums = List.of(1, 2, 3);
        int t = total(nums);        // passes a reference to the same list
        System.out.println(t + " " + nums);
    }
}
