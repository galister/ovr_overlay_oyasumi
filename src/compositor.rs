use std::ffi::CStr;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::{errors::EVRCompositorError, sys, Context};

pub struct CompositorManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRCompositor>,
}

type Result<T> = std::result::Result<T, EVRCompositorError>;

impl<'c> CompositorManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRCompositor().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    pub fn get_last_poses(
        &mut self,
        poses: &mut [sys::TrackedDevicePose_t],
        game_pose: &mut [sys::TrackedDevicePose_t],
    ) -> Result<()> {
        let err = unsafe {
            self.inner.as_mut().GetLastPoses(
                poses.as_mut_ptr(),
                poses.len() as u32,
                game_pose.as_mut_ptr(),
                game_pose.len() as u32,
            )
        };
        EVRCompositorError::new(err)
    }

    pub fn get_tracking_space(&mut self) -> sys::ETrackingUniverseOrigin {
        unsafe { self.inner.as_mut().GetTrackingSpace() }
    }

    pub fn get_frame_time_remaining(&mut self) -> f32 {
        unsafe { self.inner.as_mut().GetFrameTimeRemaining() }
    }

    pub fn get_current_scene_focus_process(&mut self) -> u32 {
        unsafe { self.inner.as_mut().GetCurrentSceneFocusProcess() }
    }

    pub fn get_last_frame_renderer(&mut self) -> u32 {
        unsafe { self.inner.as_mut().GetLastFrameRenderer() }
    }

    pub fn is_current_scene_focus_app_loading(&mut self) -> bool {
        unsafe { self.inner.as_mut().IsCurrentSceneFocusAppLoading() }
    }

    pub fn get_vulkan_instance_extensions_required(&mut self) -> Vec<String> {
        let mut buf = [0i8; 1024];
        let len = unsafe {
            self.inner
                .as_mut()
                .GetVulkanInstanceExtensionsRequired(buf.as_mut_ptr(), buf.len() as u32)
        };
        if len == 0 {
            return vec![];
        }
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
        let s = cstr.to_str().unwrap();
        s.split(' ').map(|s| s.to_owned()).collect()
    }

    pub fn get_vulkan_device_extensions_required(&mut self, device: u64) -> Vec<String> {
        let mut buf = [0i8; 1024];
        let mut handle = device;
        unsafe {
            let len = self.inner.as_mut().GetVulkanDeviceExtensionsRequired(
                (&mut handle) as *mut u64 as *mut _,
                buf.as_mut_ptr(),
                buf.len() as u32,
            );
            if len == 0 {
                return vec![];
            }
        }
        let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };
        let s = cstr.to_str().unwrap();
        s.split(' ').map(|s| s.to_owned()).collect()
    }
}
