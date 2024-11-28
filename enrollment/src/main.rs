use windows::{
    core::{w, HSTRING},
    Win32::Management::MobileDeviceManagementRegistration::RegisterDeviceWithManagement,
};

fn main() {
    match unsafe {
        RegisterDeviceWithManagement(
            w!(""), // Can be empty since we're authenticating through access token
            &HSTRING::from("https://mdmwindows.com/EnrollmentServer/Discovery.svc"),
            &HSTRING::from(""), // TODO
        )
    } {
        Ok(_) => {}
        Err(error) => {
            println!("Error while enrolling to MDM {:?}", error);
        }
    }
}
