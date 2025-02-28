// Updating the signature of Config::build to expect an iterator
// This still won't compile beacause we need to update the function body

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--
