public class Main {
    public static void main(String[] args) {
        final var name = "gopher";   // final: cannot change
        var count = 1;               // locals can change by default
        count += 1;

        count = count * 10;          // a local cannot reuse a local's name in Java: reassign instead
        System.out.println(name + " " + count);
    }
}
