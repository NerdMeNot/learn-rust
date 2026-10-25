public class Main {
    static int sum(int... xs) {
        int total = 0;
        for (int x : xs) {
            total += x;
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println("sum() = " + sum());
        System.out.println("sum(1, 2, 3, 4) = " + sum(1, 2, 3, 4));
    }
}
