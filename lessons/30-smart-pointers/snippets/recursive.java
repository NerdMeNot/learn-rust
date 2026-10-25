public class Main {
    // a field of class type is a reference, so a class can hold another of itself
    static class Expr {
        int value;
        Expr left, right;

        Expr(int value, Expr left, Expr right) {
            this.value = value;
            this.left = left;
            this.right = right;
        }
    }

    public static void main(String[] args) {
        Expr tree = new Expr(1, new Expr(2, null, null), new Expr(3, null, null));
        System.out.println("left child: " + tree.left.value);
    }
}
