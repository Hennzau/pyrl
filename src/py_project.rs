pub mod config;

use config::{
    Version, Package
};

use std::path::Path;
use std::fs::File;
use std::fs;

pub struct PyProject {
    pub name: String,
    pub py_version: Version,
    pub dependencies: Option<Vec<Package>>
}

impl PyProject {
    /*
        This function will create a fresh **new** python node if no folder entitled 'name' exists
        If it already exists, it will load a PyProject linked to the existing node
     */

    pub fn new(name: String, py_version: Version, dependencies: Option<Vec<Package>>) -> Self {
        if Path::new(name.as_str()).exists() {
            return PyProject::load(name);
        }

        let root = name.clone() + "/";
        let python = format!("{}/src/{}/", root, name);

        fs::create_dir_all(python.clone()).expect("Could not create python node");
        File::create(root + "pyrl.toml").expect("Could not create pyrl.toml file");
        File::create(python.clone() + name.as_str() + ".py").expect("Could not create python node .py file");
        File::create(python + "__init__.py").expect("Could not create python node __init__.py file");

        /*
            Write basic code in those files depending if it's a node or an operator
         */

        Self { name, py_version, dependencies }
    }

    /*
        This function will check if a directory entitled 'name' exists, if so it will load this as a project
     */

    pub fn load(name: String) -> Self {
        if Path::new(name.as_str()).exists() {
            return Self { name, py_version: Version::none(), dependencies: None }
        }

        Self { name, py_version: Version::none(), dependencies: None }
    }
}