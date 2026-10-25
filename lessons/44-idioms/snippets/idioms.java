public class Main {
    // A builder: a second class that collects the settings, with one build() that checks them.
    record Request(String url, String method, int retries) {
        static class Builder {
            private String url;
            private String method = "GET";
            private int retries = 3;

            Builder url(String url) { this.url = url; return this; }
            Builder method(String method) { this.method = method; return this; }
            Builder retries(int retries) { this.retries = retries; return this; }

            Request build() {
                if (url == null) throw new IllegalStateException("missing url");
                return new Request(url, method, retries);
            }
        }
    }

    public static void main(String[] args) {
        Request r = new Request.Builder().url("https://example.org").method("POST").retries(0).build();
        System.out.println(r.method() + " " + r.url() + " retries=" + r.retries());
        try {
            new Request.Builder().retries(5).build();
        } catch (IllegalStateException e) {
            System.out.println("error: " + e.getMessage());
        }
    }
}
