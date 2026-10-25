public class Main {
    static void log(String msg) {
        System.out.println("log: " + msg);
    }

    public static void main(String[] args) {
        var r = log("hello");
        System.out.println(r);
    }
}
