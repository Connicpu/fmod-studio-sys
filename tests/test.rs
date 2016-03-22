extern crate fmod_studio_sys as fmod;

use std::ptr;
use fmod::*;

#[test]
fn load_fmod() {
    unsafe {
        let mut system = ptr::null_mut();
        
        let result = FMOD_Studio_System_Create(
            &mut system,
            FMOD_VERSION,
        );
        assert_eq!(result, FMOD_RESULT::FMOD_OK);
        
        let result = FMOD_Studio_System_Initialize(
            system,
            128,
            0, 0,
            ptr::null_mut(),
        );
        assert_eq!(result, FMOD_RESULT::FMOD_OK);
        
        let result = FMOD_Studio_System_Release(
            system
        );
        assert_eq!(result, FMOD_RESULT::FMOD_OK);
    }
}
