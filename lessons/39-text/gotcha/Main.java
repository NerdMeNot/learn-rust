public class Main {
    public static void main(String[] args) {
        String line = "ana,30,lisbon";
        String[] parts = line.split(",");
        String name = parts[0];
        System.out.println(name);
    }
}
