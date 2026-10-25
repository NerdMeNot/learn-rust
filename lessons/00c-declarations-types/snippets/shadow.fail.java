public class Main {
    public static void main(String[] args) {
        int x = 1;
        {
            int x = x * 10; // a local cannot be declared twice, even in an inner block
            System.out.println(x);
        }
    }
}
