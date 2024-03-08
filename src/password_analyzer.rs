use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};
use tabled::{
    builder::Builder,
    settings::{Panel, Style},
};
use zxcvbn::zxcvbn;

pub struct PasswordAnalysis<'a> {
    password: &'a str,
    entropy: zxcvbn::Entropy,
}

impl Serialize for PasswordAnalysis<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut crack_times = HashMap::new();

        crack_times.insert(
            "100/h",
            self.entropy
                .crack_times()
                .online_throttling_100_per_hour()
                .to_string(),
        );

        crack_times.insert(
            "10/s",
            self.entropy
                .crack_times()
                .online_no_throttling_10_per_second()
                .to_string(),
        );

        crack_times.insert(
            "10^4/s",
            self.entropy
                .crack_times()
                .offline_slow_hashing_1e4_per_second()
                .to_string(),
        );

        crack_times.insert(
            "10^10/s",
            self.entropy
                .crack_times()
                .offline_fast_hashing_1e10_per_second()
                .to_string(),
        );

        let mut struct_serializer = serializer.serialize_struct("SecurityAnalysis", 3)?;

        struct_serializer.serialize_field(
            "strength",
            &PasswordStrength::from(self.entropy.score()).to_string(),
        )?;

        struct_serializer.serialize_field(
            "guesses",
            format!("10^{:.0}", &self.entropy.guesses_log10()).as_str(),
        )?;

        struct_serializer.serialize_field("crack_times", &crack_times)?;

        struct_serializer.end()
    }
}

impl<'a> PasswordAnalysis<'a> {
    pub fn new(password: &'a str) -> Self {
        let entropy = zxcvbn(password, &[]).expect("unable to analyze password");

        Self { password, entropy }
    }

    pub fn display_password_report(&self) {
        self.display_password_table();
        self.display_password_analysis_table();
        self.display_password_crack_time_table();
    }

    fn display_password_table(&self) {
        let mut builder = Builder::new();
        builder.push_record(vec![self.password]);

        let mut table = builder.build();
        table
            .with(Panel::header("Generated Password"))
            .with(Style::modern());

        println!("{}", table);
    }

    fn display_password_analysis_table(&self) {
        let mut builder = Builder::new();
        builder.push_record(vec![
            "Strength",
            &PasswordStrength::from(self.entropy.score()).to_string(),
        ]);

        builder.push_record(vec![
            "Guesses",
            format!("{}", &self.entropy.guesses_log10()).as_str(),
        ]);

        let mut table = builder.build();
        table
            .with(Panel::header("Password Security Analysis"))
            .with(Style::modern());

        println!("{}", table);
    }

    fn display_password_crack_time_table(&self) {
        let mut builder = Builder::new();
        builder.push_record(vec![
            "100 attempts/hour",
            format!(
                "{}",
                self.entropy.crack_times().online_throttling_100_per_hour()
            )
            .as_str(),
        ]);

        builder.push_record(vec![
            "10 attempts/second",
            format!(
                "{}",
                self.entropy
                    .crack_times()
                    .online_no_throttling_10_per_second()
            )
            .as_str(),
        ]);

        builder.push_record(vec![
            "10^4 attempts/second",
            format!(
                "{}",
                self.entropy
                    .crack_times()
                    .offline_slow_hashing_1e4_per_second()
            )
            .as_str(),
        ]);

        builder.push_record(vec![
            "10^10 attempts/second",
            format!(
                "{}",
                self.entropy
                    .crack_times()
                    .offline_fast_hashing_1e10_per_second()
            )
            .as_str(),
        ]);

        let mut table = builder.build();
        table
            .with(Panel::header("Password Crack Time Estimations"))
            .with(Style::modern());

        println!("{}", table);
    }
}

enum PasswordStrength {
    VeryWeak,
    Weak,
    Good,
    Strong,
    VeryStrong,
}

impl From<u8> for PasswordStrength {
    fn from(score: u8) -> Self {
        match score {
            0 => PasswordStrength::VeryWeak,
            1 => PasswordStrength::Weak,
            2 => PasswordStrength::Good,
            3 => PasswordStrength::Strong,
            4 => PasswordStrength::VeryStrong,
            _ => panic!("invalid score"),
        }
    }
}

impl Display for PasswordStrength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strength = match self {
            PasswordStrength::VeryWeak => "very weak",
            PasswordStrength::Weak => "weak",
            PasswordStrength::Good => "good",
            PasswordStrength::Strong => "strong",
            PasswordStrength::VeryStrong => "very strong",
        };

        write!(f, "{}", strength)
    }
}

#[cfg(test)]
mod tests {
    use super::{PasswordAnalysis, PasswordStrength};

    #[test]
    fn password_strength_test() {
        let veryweak_password = "zxcvbn";
        let veryweak_analyzer = PasswordAnalysis::new(&veryweak_password);
        assert_eq!(
            PasswordStrength::from(veryweak_analyzer.entropy.score()).to_string(),
            "very weak"
        );

        let weak_password = "!zxcvbn";
        let weak_analyzer = PasswordAnalysis::new(&weak_password);
        assert_eq!(
            PasswordStrength::from(weak_analyzer.entropy.score()).to_string(),
            "weak"
        );

        let good_password = "Tr0ub4dour&3";
        let good_analyzer = PasswordAnalysis::new(&good_password);
        assert_eq!(
            PasswordStrength::from(good_analyzer.entropy.score()).to_string(),
            "good"
        );

        let strong_password = "BTYAFpgVxN";
        let strong_analyzer = PasswordAnalysis::new(&strong_password);
        assert_eq!(
            PasswordStrength::from(strong_analyzer.entropy.score()).to_string(),
            "strong"
        );

        let verystrong_password = "s`4V~74HzxOA";
        let verystrong_analyzer = PasswordAnalysis::new(&verystrong_password);
        assert_eq!(
            PasswordStrength::from(verystrong_analyzer.entropy.score()).to_string(),
            "very strong"
        );
    }
}
