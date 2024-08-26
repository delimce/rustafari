use os_info;

trait SoftwareInfo {
    fn get_os_type(&self) -> String;
    fn get_os_version(&self) -> String;
    fn get_os_name(&self) -> String;
    fn get_so_architecture(&self) -> String;
}

pub struct Software;

impl SoftwareInfo for Software {
    fn get_os_type(&self) -> String {
        let info = os_info::get();
        info.os_type().to_string()
    }

    fn get_os_version(&self) -> String {
        sys_info::os_release().unwrap()
    }

    fn get_os_name(&self) -> String {
        match sys_info::linux_os_release() {
            Ok(value) => return value.pretty_name.unwrap(),
            Err(e) => {
                let info = os_info::get();
                return info.version().to_string();
            }
        };
    }

    fn get_so_architecture(&self) -> String {
        let info = os_info::get();
        info.architecture().unwrap().to_string()
    }
}

pub fn get_os_name() -> String {
    Software.get_os_name()
}

pub fn get_os_version() -> String {
    Software.get_os_version()
}

pub fn get_os_type() -> String {
    Software.get_os_type()
}

pub fn get_so_architecture() -> String {
    Software.get_so_architecture()
}
