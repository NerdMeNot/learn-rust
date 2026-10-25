public class Cause {
    public static void main(String[] args) {
        try {
            try {
                Integer.parseInt("eighty");
            } catch (NumberFormatException e) {
                throw new RuntimeException("reading bad.conf", e);
            }
        } catch (RuntimeException e) {
            System.out.println(e.getMessage() + ": " + e.getCause().getMessage());
        }
    }
}
