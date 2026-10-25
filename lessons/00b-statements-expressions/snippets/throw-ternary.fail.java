public class Main {
    public static void main(String[] args) {
        boolean ok = args.length == 0;
        int x = ok ? 5 : throw new IllegalStateException("not ok");
        System.out.println(x);
    }
}
