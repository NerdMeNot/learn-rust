use report::report;

#[test]
fn weekly_report() {
    let text = report(&[("website", 12.5), ("api", 7.5), ("docs", 3.0)]);
    insta::assert_snapshot!(text);
}
