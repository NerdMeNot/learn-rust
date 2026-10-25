import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] ages = {30, 40, 50};
        for (int age : ages) {
            age++;
        }
        System.out.println(Arrays.toString(ages));
    }
}
