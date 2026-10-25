public class Main {
    static class Account {
        final String owner;
        final int balance;

        Account(String owner) {
            this(owner, 0); // one constructor calls another
        }

        Account(String owner, int balance) {
            this.owner = owner;
            this.balance = balance;
        }
    }

    public static void main(String[] args) {
        var a = new Account("ana");
        var b = new Account("bo", 500);
        System.out.println(a.owner + " " + a.balance);
        System.out.println(b.owner + " " + b.balance);
    }
}
