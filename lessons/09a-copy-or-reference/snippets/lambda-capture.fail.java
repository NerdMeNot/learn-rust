public class Main {
    public static void main(String[] args) {
        int count = 0;
        Runnable bump = () -> count++;
        bump.run();
        System.out.println(count);
    }
}
