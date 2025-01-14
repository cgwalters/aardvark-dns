use clap::Parser;
use std::fmt;
use std::io::Error;

#[derive(Parser, Debug)]
pub struct Version {}

#[derive(Debug)]
struct Info {
    version: &'static str,
    commit: &'static str,
    build_time: &'static str,
    target: &'static str,
}

// since we do not need a json library here we just create the json output manually
impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{
  \"version\": \"{}\",
  \"commit\": \"{}\",
  \"build_time\": \"{}\",
  \"target\": \"{}\"
}}",
            self.version, self.commit, self.build_time, self.target
        )
    }
}

impl Version {
    pub fn exec(&self) -> Result<(), Error> {
        let info = Info {
            version: env!("VERGEN_BUILD_SEMVER"),
            commit: env!("VERGEN_GIT_SHA"),
            build_time: env!("VERGEN_BUILD_TIMESTAMP"),
            target: env!("VERGEN_RUSTC_HOST_TRIPLE"),
        };
        println!("{}", info);

        Ok(())
    }
}
