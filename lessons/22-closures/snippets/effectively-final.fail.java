public class Main {
    public static void main(String[] args) {
        int x = 1;
        Runnable show = () -> System.out.println("x: " + x);
        x = 2;
        show.run();
    }
}
