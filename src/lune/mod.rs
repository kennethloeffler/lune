use std::process::ExitCode;

mod error;
mod scheduler;

use self::scheduler::Scheduler;

pub use error::LuneError;

#[derive(Clone, Debug, Default)]
pub struct Lune {
    args: Vec<String>,
}

impl Lune {
    /**
        Creates a new Lune script runner.
    */
    pub fn new() -> Self {
        Self::default()
    }

    /**
        Arguments to give in `process.args` for a Lune script.
    */
    pub fn with_args<V>(mut self, args: V) -> Self
    where
        V: Into<Vec<String>>,
    {
        self.args = args.into();
        self
    }

    /**
        Runs a Lune script inside of a new Luau VM.
    */
    pub async fn run(
        &self,
        script_name: impl AsRef<str>,
        script_contents: impl AsRef<[u8]>,
    ) -> Result<ExitCode, LuneError> {
        Ok(Scheduler::new()
            .run_main(script_name, script_contents)
            .await)
    }
}
