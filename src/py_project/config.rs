pub struct Version {
    pub major: Option<u8>,
    pub minor: Option<u8>,
    pub patch: Option<u8>
}

impl Version {
    pub fn new(major: Option<u8>, minor: Option<u8>, patch: Option<u8>) -> Self {
        Self {
            major,
            minor,
            patch
        }
    }

    pub fn none() -> Self {
        Self {
            major: None,
            minor: None,
            patch: None,
        }
    }

    pub fn from(str: String) -> Self {
        let versions_ = str.split('.');
        let mut versions: Vec<String> = vec![];

        for version in versions_ {
            versions.push(version.to_string());
        }

        return match versions.len() {
            0 => {
                Version::none()
            },
            1 => {
                let major = versions[0].parse::<u8>();

                Version::new(major.ok(), None, None)
            },
            2 => {
                let major = versions[0].parse::<u8>();
                let minor = versions[1].parse::<u8>();

                Version::new(major.ok(), minor.ok(), None)
            },
            3 => {
                let major = versions[0].parse::<u8>();
                let minor = versions[1].parse::<u8>();
                let patch = versions[2].parse::<u8>();

                Version::new(major.ok(), minor.ok(), patch.ok())
            },
            _ => {
                Version::none()
            }
        };
    }
}

pub struct Package {
    name: String,
    version: Option<Version>
}

