fn main() {
    let line = "naïve café, naïve bar";

    // match_indices gives every match with its byte position.
    println!(
        "match_indices: {:?}",
        line.match_indices("naïve").collect::<Vec<_>>()
    );

    // The last match, then the text on either side of it.
    if let Some(at) = line.rfind("naïve") {
        println!("before: {:?}", &line[..at]);
        println!("from:   {:?}", &line[at..]);
    }

    // find returns bytes. A column for a person is a count of chars.
    let at = line.find("café").unwrap();
    println!("byte offset: {at}");
    println!("char column: {}", line[..at].chars().count());

    // replace takes a pattern too; replacen stops after n.
    println!("replace: {:?}", line.replace("naïve", "naive"));
    println!("replacen: {:?}", line.replacen("naïve", "naive", 1));
    println!("closure: {:?}", line.replace(|c: char| !c.is_ascii(), "?"));

    // A file extension is the part after the LAST dot.
    let file = "archive.tar.gz";
    println!("rsplit_once('.'): {:?}", file.rsplit_once('.'));
    println!(
        "find('.') slice:  {:?}",
        &file[file.find('.').unwrap() + 1..]
    );
}
