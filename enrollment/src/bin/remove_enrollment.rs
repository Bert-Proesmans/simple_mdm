use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::Management::MobileDeviceManagementRegistration::UnregisterDeviceWithManagement,
};

fn main() {
    match unsafe { UnregisterDeviceWithManagement(
        PCWSTR::null() // Specifically null to remove current/active/only-one registration
    ) } {
        Ok(_) => {}
        Err(error) => {
            println!("Error while enrolling to MDM {:?}", error);
        }
    }
}
