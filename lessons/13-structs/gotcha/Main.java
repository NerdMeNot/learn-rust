public class Main {
    static class User {
        String name;
        int age;
        boolean active;
    }

    public static void main(String[] args) {
        var u = new User();
        u.name = "ana";
        System.out.println(u.name + " " + u.age + " " + u.active);
    }
}
