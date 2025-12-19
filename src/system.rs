use byteorder::ByteOrder;
use slice_of_array::SliceArrayExt;
use sys::{ETrackingUniverseOrigin, HmdMatrix34_t};

use crate::errors::ETrackedPropertyError;
use crate::{sys, Context, TrackedDeviceIndex};

use std::ffi::CString;
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr::null_mut;

pub struct SystemManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRSystem>,
}

mod private {
    pub trait Sealed {}
}

type PropResult<T> = Result<T, ETrackedPropertyError>;

/// Trait implemented by types that represent storage types of properties.
pub trait TrackedDeviceProperty<'ret>: private::Sealed + Sized {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self>;
}

macro_rules! impl_property_type {
    ($ty:ty, $method:ident) => {
        impl private::Sealed for $ty {}
        impl<'ret> TrackedDeviceProperty<'ret> for $ty {
            fn get<'manager: 'ret>(
                index: TrackedDeviceIndex,
                system: &'manager mut SystemManager,
                prop: sys::ETrackedDeviceProperty,
            ) -> PropResult<Self> {
                let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
                let res = unsafe { system.inner.as_mut().$method(index.0, prop, &mut err) };
                ETrackedPropertyError::new(err)?;
                Ok(res)
            }
        }
    };
}

impl_property_type!(bool, GetBoolTrackedDeviceProperty);
impl_property_type!(f32, GetFloatTrackedDeviceProperty);
impl_property_type!(i32, GetInt32TrackedDeviceProperty);
impl_property_type!(u64, GetUint64TrackedDeviceProperty);

impl private::Sealed for String {}
impl<'ret> TrackedDeviceProperty<'ret> for String {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let res = unsafe {
            get_string(|ptr, n| {
                system.inner.as_mut().GetStringTrackedDeviceProperty(
                    index.0,
                    prop.clone(),
                    ptr,
                    n,
                    &mut err,
                )
            })
        };
        ETrackedPropertyError::new(err)?;
        match res {
            Some(s) => Ok(s.into_string().unwrap()),
            None => Ok("".to_string()),
        }
    }
}

// TODO: Decide if we want to support matrix types from other libraries, like nalgebra
impl private::Sealed for crate::pose::Matrix3x4 {}
impl<'ret> TrackedDeviceProperty<'ret> for crate::pose::Matrix3x4 {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let res = unsafe {
            system
                .inner
                .as_mut()
                .GetMatrix34TrackedDeviceProperty(index.0, prop, &mut err)
        };
        ETrackedPropertyError::new(err)?;
        Ok(res.into())
    }
}

impl private::Sealed for CString {}
impl<'ret> TrackedDeviceProperty<'ret> for CString {
    fn get<'manager: 'ret>(
        index: TrackedDeviceIndex,
        system: &'manager mut SystemManager,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<Self> {
        let mut err = sys::ETrackedPropertyError::TrackedProp_Success;
        let len = unsafe {
            system.inner.as_mut().GetStringTrackedDeviceProperty(
                index.0,
                prop.clone(),
                null_mut(),
                0,
                &mut err,
            )
        };
        ETrackedPropertyError::new(err.clone())?;
        let mut data = vec![0; len as usize];
        let _len = unsafe {
            system.inner.as_mut().GetStringTrackedDeviceProperty(
                index.0,
                prop.clone(),
                data.as_mut_ptr() as *mut i8,
                len,
                &mut err,
            )
        };
        ETrackedPropertyError::new(err)?;

        Ok(CString::from_vec_with_nul(data).expect("missing nul byte from openvr!"))
    }
}

// TODO: arrays. I don't feel like dealing with them right now.

impl<'c> SystemManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRSystem().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    pub fn get_tracked_device_property<'ret, 'manager: 'ret, T: TrackedDeviceProperty<'ret>>(
        &'manager mut self,
        index: TrackedDeviceIndex,
        prop: sys::ETrackedDeviceProperty,
    ) -> PropResult<T> {
        T::get(index, self, prop)
    }

    pub fn get_controller_role_for_tracked_device_index<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> sys::ETrackedControllerRole {
        unsafe {
            self.inner
                .as_mut()
                .GetControllerRoleForTrackedDeviceIndex(index.0)
        }
    }

    pub fn get_tracked_device_class<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> sys::ETrackedDeviceClass {
        unsafe { self.inner.as_mut().GetTrackedDeviceClass(index.0) }
    }

    pub fn is_tracked_device_connected<'ret, 'manager: 'ret>(
        &'manager mut self,
        index: TrackedDeviceIndex,
    ) -> bool {
        unsafe { self.inner.as_mut().IsTrackedDeviceConnected(index.0) }
    }

    pub fn get_device_to_absolute_tracking_pose<'ret, 'manager: 'ret>(
        &'manager mut self,
        origin: ETrackingUniverseOrigin,
        predicted_seconds_to_photons_from_now: f32,
    ) -> [sys::TrackedDevicePose_t; sys::k_unMaxTrackedDeviceCount as usize] {
        let mut poses: [sys::TrackedDevicePose_t; sys::k_unMaxTrackedDeviceCount as usize] =
            unsafe { std::mem::zeroed() };
        unsafe {
            self.inner.as_mut().GetDeviceToAbsoluteTrackingPose(
                origin,
                predicted_seconds_to_photons_from_now,
                &mut poses[0],
                sys::k_unMaxTrackedDeviceCount,
            )
        };
        poses
    }

    pub fn get_raw_zero_pose_to_standing_absolute_tracking_pose<'ret, 'manager: 'ret>(
        &'manager mut self,
    ) -> HmdMatrix34_t {
        unsafe {
            self.inner
                .as_mut()
                .GetRawZeroPoseToStandingAbsoluteTrackingPose()
        }
    }

    pub fn get_time_since_last_vsync<'ret, 'manager: 'ret>(
        &'manager mut self,
        seconds_since_last_vsync: &mut f32,
        frame_counter: &mut u64,
    ) -> bool {
        unsafe {
            self.inner
                .as_mut()
                .GetTimeSinceLastVsync(seconds_since_last_vsync, frame_counter)
        }
    }

    pub fn poll_next_event<'ret, 'manager: 'ret>(&'manager mut self) -> Option<VREvent> {
        let mut event = std::mem::MaybeUninit::uninit();
        let res = unsafe {
            self.inner.as_mut().PollNextEvent(
                event.as_mut_ptr(),
                std::mem::size_of::<sys::VREvent_t>() as u32,
            )
        };
        if !res {
            return None;
        }
        let event = unsafe { event.assume_init() };
        let event = VREvent::parse(event);
        Some(event?)
    }
}

unsafe impl Send for SystemManager<'_> {}
unsafe impl Sync for SystemManager<'_> {}

const VREVENT_SIZE: usize = std::mem::size_of::<sys::VREvent_t>();

pub struct VREvent {
    pub event_type: u32,
    pub tracked_device_index: TrackedDeviceIndex,
    pub event_age_seconds: f32,
    pub data: [u8; VREVENT_SIZE - 12],
}

impl VREvent {
    fn parse(event: sys::VREvent_t) -> Option<VREvent> {
        let bytes: [u8; VREVENT_SIZE] = unsafe {
            *std::slice::from_raw_parts(
                &event as *const sys::VREvent_t as *const u8,
                std::mem::size_of::<sys::VREvent_t>(),
            )
            .as_array()
        };
        let data = &bytes[12..VREVENT_SIZE];
        let mut data_slice: [u8; VREVENT_SIZE - 12] = [0; VREVENT_SIZE - 12];
        data_slice.copy_from_slice(data);

        Some(VREvent {
            event_type: byteorder::LittleEndian::read_u32(&bytes[0..4]),
            tracked_device_index: TrackedDeviceIndex(byteorder::LittleEndian::read_u32(
                &bytes[4..8],
            )),
            event_age_seconds: byteorder::LittleEndian::read_f32(&bytes[8..12]),
            data: data_slice,
        })
    }
}

/// Helper to call OpenVR functions that return strings
unsafe fn get_string<F: FnMut(*mut std::os::raw::c_char, u32) -> u32>(
    mut f: F,
) -> Option<std::ffi::CString> {
    let n = f(std::ptr::null_mut(), 0);
    if n == 0 {
        return None;
    }
    let mut storage = Vec::new();
    storage.reserve_exact(n as usize);
    storage.resize_with(n as usize, || std::mem::MaybeUninit::zeroed().assume_init());
    let n_ = f(storage.as_mut_ptr() as *mut _, n);
    assert!(n == n_);
    storage.truncate((n - 1) as usize); // Strip trailing null
    Some(std::ffi::CString::from_vec_unchecked(storage))
}

#[cfg(test)]
mod test {
    use super::*;
    fn _compile_test(mut system: SystemManager) {
        // let _bootloader_version =
        //     system.get_tracked_device_property(TrackedDeviceIndex::HMD, props::DisplayBootloaderVersion);
        let _display_version: u64 = system
            .get_tracked_device_property(
                TrackedDeviceIndex::HMD,
                sys::ETrackedDeviceProperty::Prop_DisplayHardwareVersion_Uint64,
            )
            .unwrap();
        let _gc_image_cstring: String = system
            .get_tracked_device_property(
                TrackedDeviceIndex::HMD,
                sys::ETrackedDeviceProperty::Prop_DisplayGCImage_String,
            )
            .unwrap();
    }
}

impl std::fmt::Debug for ETrackedPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ETrackedPropertyError").finish()
    }
}
