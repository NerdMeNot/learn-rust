public class Main {
    public static void main(String[] args) {
        String crab = "🦀";
        System.out.println("Character.BYTES: " + Character.BYTES);
        System.out.println("crab.length(): " + crab.length());
        System.out.println("code points: " + crab.codePointCount(0, crab.length()));
    }
}
