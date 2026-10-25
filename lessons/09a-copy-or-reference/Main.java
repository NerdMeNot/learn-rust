import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] ages = {30, 40, 50};

        // A copy: an int loop variable holds its own number.
        for (int age : ages) {
            age++;
            System.out.println("the copy is now " + age);
        }
        System.out.println("ages after the copy loop:     " + Arrays.toString(ages));

        // The elements themselves: index into the array.
        for (int i = 0; i < ages.length; i++) {
            ages[i]++;
        }
        System.out.println("ages after the in-place loop: " + Arrays.toString(ages));
    }
}
