use crate::Stats;

impl Stats {
    /// The counts as a JSON object. Needs the `json` feature.
    ///
    /// ```
    /// let stats = wordstat::count_words("b a b")?;
    /// assert_eq!(stats.to_json(), r#"{"a":1,"b":2}"#);
    /// # Ok::<(), wordstat::Error>(())
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::json!(self.counts()).to_string()
    }
}
