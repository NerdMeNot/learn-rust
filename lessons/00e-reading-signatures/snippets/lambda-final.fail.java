public class Main {
    public static void main(String[] args) {
        int count = 0;
        Runnable r = () -> count++;
        r.run();
        System.out.println("count: " + count);
    }
}
