import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Main {
    static class Person {
        String name;
        int age;

        Person(String name, int age) {
            this.name = name;
            this.age = age;
        }

        public String toString() {
            return name + " " + age;
        }
    }

    static void reassign(Person p) {
        p = new Person("Nobody", 0); // changes the parameter, not the caller's variable
    }

    static void mutate(Person p) {
        p.age = 99; // changes the object both names lead to
    }

    public static void main(String[] args) {
        int[] ages = {30, 40};
        for (int a : ages) {
            a++;
        }
        System.out.println("int[] after for-each a++: " + Arrays.toString(ages));

        List<Person> people = new ArrayList<>(List.of(new Person("Ada", 36), new Person("Linus", 54)));
        for (Person p : people) {
            p.age++;
        }
        System.out.println("people after for-each p.age++: " + people);

        Person ada = people.get(0);
        reassign(ada);
        System.out.println("after reassign(ada): " + ada);
        mutate(ada);
        System.out.println("after mutate(ada):   " + ada);

        // clone() copies the outer array only
        Person[] team = {new Person("Bo", 20)};
        Person[] copy = team.clone();
        copy[0].age = 21;
        System.out.println("team[0] after changing the clone's element: " + team[0]);

        // A lambda captures a copy of the variable's value, so the variable may not change.
        // A one-element array is a copy of an arrow, so the object it leads to can.
        int[] box = {0};
        Runnable bump = () -> box[0]++;
        bump.run();
        bump.run();
        System.out.println("box[0] after two runs: " + box[0]);

        // Integer boxes are objects, and == compares identity
        Integer small1 = 127, small2 = 127, big1 = 1000, big2 = 1000;
        System.out.println("127 == 127: " + (small1 == small2) + ", 1000 == 1000: " + (big1 == big2));
    }
}
