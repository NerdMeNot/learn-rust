public class Reassign {
    public static void main(String[] args) {
        int count = 0;
        count += 1; // fine: locals are mutable unless you write `final`
        final int fixed = 0;
        fixed += 1;
        System.out.println(count + fixed);
    }
}
