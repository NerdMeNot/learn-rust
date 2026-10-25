import java.util.Optional;

public class Main {
    record User(String name) {}

    static Optional<User> findUser(int id) {
        return id == 1 ? Optional.of(new User("ana")) : Optional.empty();
    }

    public static void main(String[] args) {
        findUser(1).ifPresentOrElse(
            u -> System.out.println("found " + u.name()),
            () -> System.out.println("no user"));

        // map + orElse: the short form
        System.out.println(findUser(2).map(u -> "found " + u.name()).orElse("no user"));
    }
}
