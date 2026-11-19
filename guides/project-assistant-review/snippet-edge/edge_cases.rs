
// Added to the end of step 4's src/lib.rs, before its count_line is changed.
#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn a_level_word_inside_the_message_is_not_the_level() -> Result<(), String> {
        let mut stats = Stats::default();
        count_line("2026-11-03T10:00:07Z WARN payment gateway said ERROR, retrying", &mut stats)?;
        assert_eq!(stats, Stats { warn: 1, ..Stats::default() });
        Ok(())
    }

    #[test]
    fn a_field_that_ends_in_ms_is_not_the_duration() -> Result<(), String> {
        let mut stats = Stats::default();
        count_line("2026-11-03T10:00:08Z INFO batch done items=40 ms=12", &mut stats)?;
        assert_eq!(stats.slowest_ms, 12);
        Ok(())
    }
}
