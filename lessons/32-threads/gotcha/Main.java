import java.util.List;

public class Main {
    public static void main(String[] args) throws InterruptedException {
        var names = List.of("a", "b", "c");
        var t = new Thread(() -> {
            System.out.println(names.size());
        });
        t.start();
        t.join();
    }
}
