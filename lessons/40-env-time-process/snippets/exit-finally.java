public class Main {
    public static void main(String[] args) {
        try {
            System.out.println("calling System.exit");
            System.exit(0);
        } finally {
            System.out.println("finally ran");
        }
    }
}
