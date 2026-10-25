struct Player {
    name: String,
    item: Option<String>,
}

/// Move `from`'s item to `to`. Return what `to` was holding before, if anything.
/// If `from` has no item, change nothing and return None.
fn hand_over(from: &mut Player, to: &mut Player) -> Option<String> {
    // take() moves the item out and leaves None. `?` returns early on None.
    let item = from.item.take()?;
    // replace() puts the new item in and gives back the old one.
    to.item.replace(item)
}

fn main() {
    let mut ana = Player {
        name: String::from("ana"),
        item: Some(String::from("sword")),
    };
    let mut bo = Player {
        name: String::from("bo"),
        item: None,
    };
    let sword_text = ana.item.as_ref().unwrap().as_ptr();

    // ana gives bo her sword. bo had nothing, so nothing comes back.
    assert_eq!(hand_over(&mut ana, &mut bo), None);
    assert_eq!(ana.item, None);
    assert_eq!(bo.item.as_deref(), Some("sword"));
    assert_eq!(
        bo.item.as_ref().unwrap().as_ptr(),
        sword_text,
        "move the item, don't clone it"
    );

    // ana has nothing to give: bo keeps his sword.
    assert_eq!(hand_over(&mut ana, &mut bo), None);
    assert_eq!(bo.item.as_deref(), Some("sword"));

    // ana finds a shield and gives it to bo, who hands back what he was holding.
    ana.item = Some(String::from("shield"));
    let returned = hand_over(&mut ana, &mut bo);
    assert_eq!(returned.as_deref(), Some("sword"));
    assert_eq!(bo.item.as_deref(), Some("shield"));
    assert_eq!(ana.item, None);

    println!("{} and {}: all checks passed", ana.name, bo.name);
}
