import java.util.List;

public class Main {
    static <T extends Comparable<T>> T largest(List<T> list) {
        return list.get(0);
    }

    static class Base implements Comparable<Base> {
        public int compareTo(Base o) {
            return 0;
        }
    }

    static class Derived extends Base {}

    public static void main(String[] args) {
        List<Derived> items = List.of(new Derived());
        System.out.println(largest(items) != null);
    }
}
