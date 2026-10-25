/** Turns a title into a URL slug: lowercase words joined by "-". */
public class Slug {
    private Slug() {}

    /**
     * Builds the slug.
     *
     * @param title the text to convert
     * @return lowercase words joined by "-"
     */
    public static String slug(String title) {
        return String.join("-", title.trim().toLowerCase().split("\\s+"));
    }

    /**
     * Prints the slug of a fixed title.
     *
     * @param args not used
     */
    public static void main(String[] args) {
        System.out.println(slug("Hello Java World"));
    }
}
