class Counter {
    int hits = 0;
}

public class Main {
    public static void main(String[] args) {
        Counter a = new Counter();
        Counter b = a;

        a.hits++;
        b.hits++;
        System.out.println("hits: " + a.hits);
    }
}
