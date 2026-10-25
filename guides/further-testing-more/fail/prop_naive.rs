// tests/prop_naive.rs: the usual way to write it, on the naive function.
use avg::average_naive;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: None,
        rng_seed: proptest::test_runner::RngSeed::Fixed(42),
        ..ProptestConfig::default()
    })]

    #[test]
    fn naive_average_is_between_min_and_max(samples in prop::collection::vec(any::<i32>(), 1..8)) {
        let avg = average_naive(&samples).unwrap();
        prop_assert!(avg >= *samples.iter().min().unwrap());
        prop_assert!(avg <= *samples.iter().max().unwrap());
    }
}
