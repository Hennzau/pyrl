pub mod py_project;

use py_project::PyProject;
use py_project::config::Version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let operator = PyProject::new("test_operator".to_string(), Version::from("3.12".to_string()), None);

    }
}
