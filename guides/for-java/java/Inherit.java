class User {
    String name;

    User(String name) {
        this.name = name;
    }

    String describe() {
        return "user " + name;
    }
}

class Admin extends User {
    Admin(String name) {
        super(name);
    }

    @Override
    String describe() {
        return "admin " + super.describe();
    }
}

public class Inherit {
    public static void main(String[] args) {
        User u = new Admin("ana");
        System.out.println(u.describe());
        System.out.println(u.name);
    }
}
