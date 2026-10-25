public class Main {
    record Pair<T>(T first, T second) {}

    public static void main(String[] args) {
        Pair<Integer> pair = new Pair<>(1000, 1000);
        Object first = pair.first();
        System.out.println("first is a " + first.getClass().getName());
        System.out.println("first and second are the same object: " + (pair.first() == pair.second()));
    }
}
