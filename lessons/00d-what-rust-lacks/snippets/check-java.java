import java.util.Optional;
import java.util.concurrent.atomic.AtomicInteger;

public class Main {
    static class Animal {
        final String name;

        Animal(String name) {
            this.name = name;
        }
    }

    static class Dog extends Animal {
        Dog(String name) {
            super(name);
        }
    }

    interface Speak {
        String speak();
    }

    static class Cat implements Speak {
        @Override
        public String speak() {
            return "meow";
        }
    }

    static class Point {
        final int x;
        final int y;

        Point(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    static class Plain {
        int x;
        String name;
    }

    static String show(int n) {
        return "int " + n;
    }

    static String show(String s) {
        return "text " + s;
    }

    static int visits = 0;

    static int parse(String s) {
        return Integer.parseInt(s) + 1;
    }

    static void check(boolean ok, String what) {
        if (!ok) {
            throw new IllegalStateException(what);
        }
    }

    public static void main(String[] args) throws Exception {
        // ++ and --, also inside expressions
        int[] a = {10, 20, 30};
        int i = 0;
        int first = a[i++];
        int second = a[++i];
        check(first == 10 && second == 30 && i == 2, "inc");
        int j = 5;
        int pre = --j;
        check(pre == 4 && j == 4, "--x decrements");

        // the ternary operator
        String label = i > 1 ? "big" : "small";
        check(label.equals("big"), "ternary");

        // null and Optional
        String name = null;
        check(name == null, "null");
        check(Optional.empty().isEmpty(), "optional");

        // new and this
        Point p = new Point(1, 2);
        check(p.x == 1 && p.y == 2, "new");

        // fields start at zero and null
        Plain plain = new Plain();
        check(plain.x == 0 && plain.name == null, "zero fields");

        // overloads
        check(show(1).equals("int 1") && show("a").equals("text a"), "overload");

        // old-style switch falls through without break
        StringBuilder out = new StringBuilder();
        switch (1) {
            case 1:
                out.append("a");
            case 2:
                out.append("b");
                break;
            default:
                out.append("c");
        }
        check(out.toString().equals("ab"), "fallthrough");

        // exceptions, and finally
        StringBuilder trace = new StringBuilder();
        try {
            parse("x");
            check(false, "no exception");
        } catch (NumberFormatException e) {
            trace.append("caught,");
        } finally {
            trace.append("finally");
        }
        check(trace.toString().equals("caught,finally") && parse("41") == 42, "catch");

        // inheritance and implements
        Animal d = new Dog("Rex");
        check(d.name.equals("Rex") && new Cat().speak().equals("meow"), "inherit");

        // int to long needs no cast, and a cast has parentheses
        long big = i;
        long cast = (long) i;
        check(big == 2L && cast == 2L, "widen");

        // class-level mutable state, and an atomic counter
        visits++;
        AtomicInteger counter = new AtomicInteger();
        counter.getAndAdd(1);
        check(visits == 1 && counter.get() == 1, "global");

        System.out.println("java: all checks passed");
    }
}
