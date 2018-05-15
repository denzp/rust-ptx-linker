use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate_compile_test::prelude::*;

pub mod assembly;
pub mod ir;

trait LinkOutputCheckStep {
    fn get_crate_name(&self) -> String;
    fn get_content(&self, profile: &Profile, path: &str) -> Option<(&[&str], &[&str])>;

    fn check_output(&self, config: &Config, path: &Path) -> Result<()> {
        match self.get_content(&config.profile, &self.get_crate_name()) {
            Some((expected, not_expected)) => {
                let mut contents = String::new();
                let mut file = BufReader::new(File::open(path)?);

                file.read_to_string(&mut contents)?;

                for item in expected {
                    if !contents.contains(item) {
                        bail!("File {:?} should contain {:?}", path, item);
                    }
                }

                for item in not_expected {
                    if contents.contains(item) {
                        bail!("File {:?} should not contain {:?}", path, item);
                    }
                }
            }

            None => {
                bail!(
                    "Unexpected crate `{}` and profile `{}`",
                    self.get_crate_name(),
                    if config.profile == Profile::Debug {
                        "debug"
                    } else {
                        "release"
                    }
                );
            }
        }

        Ok(())
    }
}
