public class Main {
    record User(String name) {}

    static User findUser(int id) {
        return id == 1 ? new User("ana") : null;
    }

    public static void main(String[] args) {
        var u = findUser(2);
        System.out.println(u.name());
    }
}
