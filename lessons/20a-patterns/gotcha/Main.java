public class Main {
    public static void main(String[] args) {
        String line = "mode";
        String[] parts = line.split("=");
        System.out.println(parts[0] + " -> " + parts[1]);
    }
}
