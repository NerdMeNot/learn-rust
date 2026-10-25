public class Main {
    // One class, two Comparable<...>: Java refuses.
    static class Length implements Comparable<Length>, Comparable<String> {
        public int compareTo(Length other) {
            return 0;
        }

        public int compareTo(String other) {
            return 0;
        }
    }

    public static void main(String[] args) {
        System.out.println(new Length().compareTo("x"));
    }
}
