public class Main {
    // a record gets a constructor, getters and toString for free
    record User(String name, int age, boolean active) {}

    public static void main(String[] args) {
        var u = new User("ana", 30, true);
        System.out.println(u.name() + " is " + u.age() + ", active=" + u.active());
        System.out.println(u);
    }
}
