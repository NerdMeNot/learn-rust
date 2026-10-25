public class Main {
    record User(String name, int age) {}

    public static void main(String[] args) {
        var a = new User("ana", 30);
        var b = new User("ana", 30);
        System.out.println("a.equals(b): " + a.equals(b));
        System.out.println("a == b: " + (a == b));
        System.out.println("same hashCode: " + (a.hashCode() == b.hashCode()));
        System.out.println("toString: " + a);
    }
}
