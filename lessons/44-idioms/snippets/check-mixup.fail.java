public class Main {
    record UserId(int value) {}
    record OrderId(int value) {}

    static String loadUser(UserId id) {
        return "user #" + id.value();
    }

    public static void main(String[] args) {
        OrderId order = new OrderId(7);
        System.out.println(loadUser(order));
    }
}
