use crate::{errors::EVRInputError, pose, sys, Context};

use derive_more::{From, Into};
use enumset::{EnumSet, EnumSetType};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::Path;
use std::pin::Pin;
use std::time::Duration;

pub struct InputManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRInput>,
}

#[derive(From, Into, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct ActionSetHandle(pub sys::VRActionSetHandle_t);

#[derive(From, Into, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct ActionHandle(sys::VRActionHandle_t);

#[derive(From, Into, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct InputValueHandle(pub sys::VRInputValueHandle_t);

#[derive(From, Into /*, Debug, PartialEq, Eq, Clone, Copy*/)]
#[repr(transparent)]
// TODO: do we want to do something else to forward fields to the sys struct?
pub struct ActiveActionSet(pub sys::VRActiveActionSet_t);

#[derive(From, Into /*, Debug, PartialEq, Eq, Clone, Copy*/)]
#[repr(transparent)]
pub struct DigitalActionData(pub sys::InputDigitalActionData_t);

#[derive(From, Into /*, Debug, PartialEq, Eq, Clone, Copy*/)]
#[repr(transparent)]
pub struct AnalogActionData(pub sys::InputAnalogActionData_t);

#[derive(From, Into /*, Debug, PartialEq, Eq, Clone, Copy*/)]
#[repr(transparent)]
pub struct PoseActionData(pub sys::InputPoseActionData_t);

#[derive(From, Into /*, Debug, PartialEq, Eq, Clone, Copy*/)]
#[repr(transparent)]
pub struct OriginInfo(pub sys::InputOriginInfo_t);

type Result<T> = std::result::Result<T, EVRInputError>;

pub trait ToSeconds {
    fn to_seconds(self) -> f32;
}

impl ToSeconds for f32 {
    fn to_seconds(self) -> f32 {
        self
    }
}

impl ToSeconds for &f32 {
    fn to_seconds(self) -> f32 {
        *self
    }
}

impl ToSeconds for &Duration {
    fn to_seconds(self) -> f32 {
        self.as_secs_f32()
    }
}

#[derive(EnumSetType, Debug)]
#[enumset(repr = "u32")]
pub enum InputString {
    Hand,
    ControllerType,
    InputSource,
    // TODO: openvr allows you to pass a u32 with all bits set to get a string that has all information, current and future.
    //       is there a good way to represent that with enumset? do we care?
}

impl<'c> InputManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRInput().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    // ---- Handle Management ----

    pub fn set_action_manifest(&mut self, path: &Path) -> Result<()> {
        let path = if let Ok(s) = CString::new(path.to_string_lossy().as_bytes()) {
            s
        } else {
            return EVRInputError::new(sys::EVRInputError::VRInputError_InvalidParam);
        };
        self.set_action_manifest_raw(&path)
    }

    pub fn set_action_manifest_raw(&mut self, path: &CStr) -> Result<()> {
        let err = unsafe { self.inner.as_mut().SetActionManifestPath(path.as_ptr()) };
        EVRInputError::new(err)
    }

    pub fn get_action_set_handle(&mut self, name: &str) -> Result<ActionSetHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            return Err(sys::EVRInputError::VRInputError_InvalidParam.into());
        };

        self.get_action_set_handle_raw(&name)
    }

    pub fn get_action_set_handle_raw(&mut self, name: &CStr) -> Result<ActionSetHandle> {
        let mut handle: sys::VRActionSetHandle_t = 0;

        let err = unsafe {
            self.inner
                .as_mut()
                .GetActionSetHandle(name.as_ptr(), &mut handle)
        };

        EVRInputError::new(err)?;
        Ok(ActionSetHandle(handle))
    }

    pub fn get_action_handle(&mut self, name: &str) -> Result<ActionHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            return EVRInputError::new(sys::EVRInputError::VRInputError_InvalidParam)
                .map(|_| unreachable!());
        };

        self.get_action_handle_raw(&name)
    }

    pub fn get_action_handle_raw(&mut self, name: &CStr) -> Result<ActionHandle> {
        let mut handle: sys::VRActionHandle_t = 0;

        let err = unsafe {
            self.inner
                .as_mut()
                .GetActionHandle(name.as_ptr(), &mut handle)
        };

        EVRInputError::new(err)?;
        Ok(ActionHandle(handle))
    }

    pub fn get_input_source_handle(&mut self, name: &str) -> Result<InputValueHandle> {
        let name = if let Ok(s) = CString::new(name) {
            s
        } else {
            return EVRInputError::new(sys::EVRInputError::VRInputError_InvalidParam)
                .map(|_| unreachable!());
        };

        self.get_input_source_handle_raw(&name)
    }

    pub fn get_input_source_handle_raw(&mut self, name: &CStr) -> Result<InputValueHandle> {
        let mut handle: sys::VRInputValueHandle_t = 0;

        let err = unsafe {
            self.inner
                .as_mut()
                .GetInputSourceHandle(name.as_ptr(), &mut handle)
        };

        EVRInputError::new(err)?;
        Ok(InputValueHandle(handle))
    }

    // ---- Read Action State ----

    pub fn update_actions(&mut self, sets: &mut [ActiveActionSet]) -> Result<()> {
        let err = unsafe {
            self.inner.as_mut().UpdateActionState(
                // this should be fine because of repr(transparent)
                // TODO: have bytemuck say it's fine or something?
                sets.as_mut_ptr() as *mut sys::VRActiveActionSet_t,
                std::mem::size_of::<sys::VRActiveActionSet_t>() as u32,
                sets.len() as u32,
            )
        };

        EVRInputError::new(err)
    }

    pub fn get_digital_action_data(
        &mut self,
        action: ActionHandle,
        restrict: InputValueHandle,
    ) -> Result<DigitalActionData> {
        let mut data: MaybeUninit<sys::InputDigitalActionData_t> = MaybeUninit::uninit();
        let err = unsafe {
            self.inner.as_mut().GetDigitalActionData(
                action.0,
                data.as_mut_ptr(),
                std::mem::size_of::<sys::InputDigitalActionData_t>() as u32,
                restrict.0,
            )
        };
        EVRInputError::new(err)?;
        Ok(DigitalActionData(unsafe { data.assume_init() }))
    }

    pub fn get_analog_action_data(
        &mut self,
        action: ActionHandle,
        restrict: InputValueHandle,
    ) -> Result<AnalogActionData> {
        let mut data: MaybeUninit<sys::InputAnalogActionData_t> = MaybeUninit::uninit();
        let err = unsafe {
            self.inner.as_mut().GetAnalogActionData(
                action.0,
                data.as_mut_ptr(),
                std::mem::size_of::<sys::InputAnalogActionData_t>() as u32,
                restrict.0,
            )
        };
        EVRInputError::new(err)?;
        Ok(AnalogActionData(unsafe { data.assume_init() }))
    }

    pub fn get_pose_action_data_relative_to_now(
        &mut self,
        action: ActionHandle,
        universe: pose::TrackingUniverseOrigin,
        seconds_from_now: impl ToSeconds,
        restrict: InputValueHandle,
    ) -> Result<PoseActionData> {
        let mut data: MaybeUninit<sys::InputPoseActionData_t> = MaybeUninit::uninit();
        let err = unsafe {
            self.inner.as_mut().GetPoseActionDataRelativeToNow(
                action.0,
                universe,
                seconds_from_now.to_seconds(),
                data.as_mut_ptr(),
                std::mem::size_of::<sys::InputPoseActionData_t>() as u32,
                restrict.0,
            )
        };

        EVRInputError::new(err)?;
        Ok(PoseActionData(unsafe { data.assume_init() }))
    }

    // ---- Action Origins ----

    pub fn get_action_origins(
        &mut self,
        action_set: ActionSetHandle,
        digital_action_handle: ActionHandle,
    ) -> Result<[u64; 16]> {
        let mut origins: [u64; 16] = unsafe { std::mem::zeroed() };
        let err = unsafe {
            self.inner.as_mut().GetActionOrigins(
                action_set.0,
                digital_action_handle.0,
                origins.as_mut_ptr(),
                16,
            )
        };
        EVRInputError::new(err)?;
        Ok(origins)
    }

    pub fn get_origin_localized_name(
        &mut self,
        origin: InputValueHandle,
        bits: EnumSet<InputString>,
    ) -> Result<String> {
        let mut name: [::std::os::raw::c_char; 128usize] = unsafe { ::std::mem::zeroed() };

        let err = unsafe {
            self.inner.as_mut().GetOriginLocalizedName(
                origin.0,
                name.as_mut_ptr(),
                128,
                bits.as_repr() as i32,
            )
        };

        EVRInputError::new(err)?;
        let trimmed_str = name
            .iter()
            .map(|&c| c as u8)
            .take_while(|&x| x != 0)
            .collect();

        Ok(String::from_utf8(trimmed_str).expect("Could not parse string from name array"))
    }

    pub fn get_origin_tracked_device_info(
        &mut self,
        origin: InputValueHandle,
    ) -> Result<OriginInfo> {
        let mut data: MaybeUninit<sys::InputOriginInfo_t> = MaybeUninit::uninit();
        let err = unsafe {
            self.inner.as_mut().GetOriginTrackedDeviceInfo(
                origin.0,
                data.as_mut_ptr(),
                std::mem::size_of::<sys::InputOriginInfo_t>() as u32,
            )
        };

        EVRInputError::new(err)?;
        Ok(OriginInfo(unsafe { data.assume_init() }))
    }

    pub fn show_action_origins(
        &mut self,
        set: ActionSetHandle,
        action: ActionHandle,
    ) -> Result<()> {
        let err = unsafe { self.inner.as_mut().ShowActionOrigins(set.0, action.0) };

        EVRInputError::new(err)
    }

    pub fn show_bindings_for_action_set(
        &mut self,
        sets: &mut [ActiveActionSet],
        origin: InputValueHandle,
    ) -> Result<()> {
        let err = unsafe {
            self.inner.as_mut().ShowBindingsForActionSet(
                sets.as_mut_ptr() as *mut sys::VRActiveActionSet_t,
                std::mem::size_of::<sys::VRActiveActionSet_t>() as u32,
                sets.len() as u32,
                origin.0,
            )
        };

        EVRInputError::new(err)
    }

    pub fn trigger_haptic_vibration_action(
        &mut self,
        action: ActionHandle,
        start_seconds_from_now: f32,
        duration: Duration,
        frequency: f32,
        amplitude: f32,
        restrict: InputValueHandle,
    ) -> Result<()> {
        let err = unsafe {
            self.inner.as_mut().TriggerHapticVibrationAction(
                action.0,
                start_seconds_from_now,
                duration.as_secs_f32(),
                frequency,
                amplitude,
                restrict.0,
            )
        };

        EVRInputError::new(err)
    }

    pub fn open_binding_ui(
        &mut self,
        app_key: Option<&str>,
        action_set: Option<ActionSetHandle>,
        input_device: InputValueHandle,
        show_on_desktop: bool,
    ) -> Result<()> {
        let app_key_cstr_ptr = app_key
            .map(|s| CString::new(s).unwrap())
            .map(|cstr| cstr.as_ptr())
            .unwrap_or(std::ptr::null());
        let action_set = match action_set {
            Some(s) => s.0,
            None => 0,
        };
        let err = unsafe {
            self.inner.as_mut().OpenBindingUI(
                app_key_cstr_ptr,
                action_set,
                input_device.0,
                show_on_desktop,
            )
        };
        EVRInputError::new(err)
    }

    pub fn get_action_binding_info(
        &mut self,
        action: ActionHandle,
    ) -> std::result::Result<Vec<sys::InputBindingInfo_t>, EVRInputError> {
        let mut data: [sys::InputBindingInfo_t; 16] = unsafe { std::mem::zeroed() };
        let mut count: MaybeUninit<u32> = MaybeUninit::uninit();

        let err: sys::EVRInputError = unsafe {
            self.inner.as_mut().GetActionBindingInfo(
                action.0,
                data.as_mut_ptr(),
                std::mem::size_of::<sys::InputBindingInfo_t>() as u32,
                16,
                count.as_mut_ptr(),
            )
        };
        let err = EVRInputError::new(err);
        if let Err(err) = err {
            return std::result::Result::Err(err);
        };

        let mut data_vec = vec![];

        for i in 0..unsafe { count.assume_init() } {
            let info = unsafe { data.get_unchecked(i as usize) };
            data_vec.push(sys::InputBindingInfo_t {
                rchDevicePathName: info.rchDevicePathName,
                rchInputPathName: info.rchInputPathName,
                rchModeName: info.rchModeName,
                rchSlotName: info.rchSlotName,
                rchInputSourceType: info.rchInputSourceType,
            });
        }

        std::result::Result::Ok(data_vec)
    }
}
