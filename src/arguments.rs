use clap::Parser;

#[derive(Parser, Debug, Default)]
#[clap(author = "Lucas Waddell", version, about = "Traffic Manager")]
pub struct Arguments {
    /// Probability of checking for deadlock
    #[clap(short, long, help = "Probability of checking for deadlock")]
    pub probability: f32,

    /// Sequence of buses
    #[clap(short, help = "Sequence of directions of buses coming to the junction")]
    pub sequence: String,
}

impl Arguments {
    pub fn validate(&self) -> Result<&Self, String> {
        if self.probability >= 1.0 || self.probability <= 0.0 {
            return Err(format!(
                "The probability {:?} must be in the exclusive range (0, 1).",
                self.probability
            ));
        }

        let valid_directions = vec!['n', 's', 'e', 'w', 'N', 'S', 'E', 'W'];

        for c in self.sequence.chars() {
            if !valid_directions.contains(&c) {
                return Err(format!(
                    "The direction {:?} contains an invalid character: '{}'",
                    self.sequence, c
                ));
            }
        }

        Ok(self)
    }
}
