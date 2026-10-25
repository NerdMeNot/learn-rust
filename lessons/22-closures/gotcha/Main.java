public class Main {
    public static void main(String[] args) {
        int count = 0;
        Runnable inc = () -> count += 1;
        inc.run();
        inc.run();
        System.out.println("count=" + count);
    }
}
