public class Main {
    public static void main(String[] args) {
        String line = "  name=Ana, age=30, city=Lisbon  ";

        String clean = line.strip(); // a new String: Java strings never share a view
        System.out.println("[" + clean + "]");

        // split on ',', strip each piece, split once on '='
        for (String field : clean.split(",")) {
            String[] kv = field.strip().split("=", 2);
            if (kv.length == 2) {
                System.out.printf("%5s -> %s%n", kv[0], kv[1]);
            }
        }

        // text to number: parseInt throws on bad input
        int age;
        try {
            age = Integer.parseInt("30");
        } catch (NumberFormatException e) {
            age = 0;
        }
        System.out.println("next year: " + (age + 1));

        String shout = clean.toUpperCase().replace(", ", " | ");
        System.out.println(shout);

        System.out.println("starts with name: " + clean.startsWith("name"));
        System.out.println("position of age: " + clean.indexOf("age")); // -1 if missing

        System.out.println(String.join("-", "build", "with", "rust"));

        // grow a string piece by piece
        StringBuilder out = new StringBuilder();
        for (int i = 1; i <= 3; i++) {
            out.append(i).append(';');
        }
        System.out.println(out);
    }
}
