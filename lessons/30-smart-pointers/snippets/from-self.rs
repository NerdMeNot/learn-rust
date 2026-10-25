use std::rc::{Rc, Weak};

struct Widget {
    id: u32,
    me: Weak<Widget>, // a link to itself, filled in while it is built
}

impl Widget {
    // the receiver says: only callable on a Widget that is inside an Rc
    fn share(self: &Rc<Self>) -> Rc<Widget> {
        Rc::clone(self)
    }
}

fn main() {
    let w = Rc::new_cyclic(|me| Widget {
        id: 7,
        me: me.clone(),
    });
    let again = w.share();
    println!("id: {}, owners: {}", again.id, Rc::strong_count(&w));
    println!("own weak link upgrades: {}", w.me.upgrade().is_some());
}
