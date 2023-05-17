use std::any::Any;
use std::error::Error;
use std::fmt::Debug;

pub mod checks;
pub mod run_requirements;

pub trait CheckResult {
    fn passed(&self) -> bool;
    fn log(&self) -> ();
}

pub trait Requirement: Debug {
    // type CheckResultType: CheckResult;
    fn check(&self) -> Result<Box<dyn CheckResult>, Box<dyn Error>>;
    fn fix(&self) -> Option<Result<(), Box<dyn Error>>> {
        None
    }
    fn fixable(&self) -> bool {
        false
    }
}

