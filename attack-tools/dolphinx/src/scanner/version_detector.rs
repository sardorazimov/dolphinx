use regex::Regex;

#[derive(Debug, Clone)]
pub struct VersionInfo {

    pub product: Option<String>,
    pub version: Option<String>,
}


pub fn detect_version(banner: &Option<String>) -> VersionInfo {

    if banner.is_none() {

        return VersionInfo {
            product: None,
            version: None
        };
    }

    let banner = banner.as_ref().unwrap();

    // SSH
    let ssh = Regex::new(r"OpenSSH[_\-]?([0-9\.p]+)").unwrap();

    if let Some(cap) = ssh.captures(banner) {

        return VersionInfo {

            product: Some("OpenSSH".into()),
            version: Some(cap[1].to_string())
        };
    }

    // nginx
    let nginx = Regex::new(r"nginx/([0-9\.]+)").unwrap();

    if let Some(cap) = nginx.captures(banner) {

        return VersionInfo {

            product: Some("nginx".into()),
            version: Some(cap[1].to_string())
        };
    }

    // Apache
    let apache = Regex::new(r"Apache/([0-9\.]+)").unwrap();

    if let Some(cap) = apache.captures(banner) {

        return VersionInfo {

            product: Some("Apache".into()),
            version: Some(cap[1].to_string())
        };
    }

    VersionInfo {

        product: None,
        version: None
    }
}
