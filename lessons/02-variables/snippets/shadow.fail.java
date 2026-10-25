public class Main {
    public static void main(String[] args) {
        int x = 1;
        {
            int x = 10;
            System.out.println(x);
        }
    }
}
