use sys::{HmdMatrix34_t, HmdQuad_t, HmdVector2_t};

use crate::{sys, Context};

use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr::{self, null_mut};

pub struct ChaperoneSetupManager<'c> {
    ctx: PhantomData<&'c Context>,
    inner: Pin<&'c mut sys::IVRChaperoneSetup>,
}

impl<'c> ChaperoneSetupManager<'c> {
    pub(super) fn new(_ctx: &'c Context) -> Self {
        let inner = unsafe { Pin::new_unchecked(sys::VRChaperoneSetup().as_mut::<'c>().unwrap()) };
        Self {
            ctx: Default::default(),
            inner,
        }
    }

    // TODO: this outputs json, could we pass it directly to something that does json?
    pub fn export_live_to_buffer(&mut self) -> Option<CString> {
        let mut len = 0u32;
        // Passing null pointer here means it will merely write to the length parameter.
        let _res = unsafe { self.inner.as_mut().ExportLiveToBuffer(null_mut(), &mut len) };
        if len == 0 {
            return None;
        }

        let mut data = vec![0u8; len as usize];
        let res = unsafe {
            self.inner
                .as_mut()
                .ExportLiveToBuffer(data.as_mut_ptr() as *mut i8, &mut len)
        };
        if res {
            CString::from_vec_with_nul(data).ok()
        } else {
            None
        }
    }

    pub fn get_working_standing_zero_pose_to_raw_tracking_pose(&mut self) -> Option<HmdMatrix34_t> {
        let mut pose = MaybeUninit::uninit();
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingStandingZeroPoseToRawTrackingPose(pose.as_mut_ptr())
        };
        if success {
            Some(unsafe { pose.assume_init() })
        } else {
            None
        }
    }

    pub fn get_working_seated_zero_pose_to_raw_tracking_pose(&mut self) -> Option<HmdMatrix34_t> {
        let mut pose = MaybeUninit::uninit();
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingSeatedZeroPoseToRawTrackingPose(pose.as_mut_ptr())
        };
        if success {
            Some(unsafe { pose.assume_init() })
        } else {
            None
        }
    }

    pub fn set_working_standing_zero_pose_to_raw_tracking_pose(&mut self, mat: &HmdMatrix34_t) {
        unsafe {
            self.inner
                .as_mut()
                .SetWorkingStandingZeroPoseToRawTrackingPose(mat)
        }
    }

    pub fn set_working_seated_zero_pose_to_raw_tracking_pose(&mut self, mat: &HmdMatrix34_t) {
        unsafe {
            self.inner
                .as_mut()
                .SetWorkingSeatedZeroPoseToRawTrackingPose(mat)
        }
    }

    pub fn get_live_collision_bounds_info(&mut self) -> Vec<HmdQuad_t> {
        let mut num_quads = 0u32;
        let success = unsafe {
            self.inner
                .as_mut()
                .GetLiveCollisionBoundsInfo(ptr::null_mut(), &mut num_quads)
        };
        if !success {
            return vec![];
        }
        let mut quads: Vec<HmdQuad_t> = Vec::with_capacity(num_quads as usize);
        let success = unsafe {
            self.inner
                .as_mut()
                .GetLiveCollisionBoundsInfo(quads.as_mut_ptr(), &mut num_quads)
        };
        if !success {
            return vec![];
        }
        quads
    }

    pub fn get_working_collision_bounds_info(&mut self) -> Vec<HmdQuad_t> {
        let mut num_quads = 0u32;
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingCollisionBoundsInfo(ptr::null_mut(), &mut num_quads)
        };
        if !success {
            return vec![];
        }
        let mut quads: Vec<HmdQuad_t> = Vec::with_capacity(num_quads as usize);
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingCollisionBoundsInfo(quads.as_mut_ptr(), &mut num_quads)
        };
        if !success {
            return vec![];
        }
        quads
    }

    pub fn set_working_collision_bounds_info(&mut self, quads: &mut [HmdQuad_t]) {
        unsafe {
            self.inner
                .as_mut()
                .SetWorkingCollisionBoundsInfo(quads.as_mut_ptr(), quads.len() as _)
        }
    }

    pub fn get_working_play_area_size(&mut self) -> Option<(f32, f32)> {
        let mut size_x = MaybeUninit::uninit();
        let mut size_y = MaybeUninit::uninit();
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingPlayAreaSize(size_x.as_mut_ptr(), size_y.as_mut_ptr())
        };
        if success {
            Some(unsafe { (size_x.assume_init(), size_y.assume_init()) })
        } else {
            None
        }
    }

    pub fn set_working_play_area_size(&mut self, size_x: f32, size_y: f32) {
        unsafe { self.inner.as_mut().SetWorkingPlayAreaSize(size_x, size_y) }
    }

    pub fn get_working_play_area_rect(&mut self) -> Option<HmdQuad_t> {
        let mut rect = MaybeUninit::uninit();
        let success = unsafe {
            self.inner
                .as_mut()
                .GetWorkingPlayAreaRect(rect.as_mut_ptr())
        };
        if success {
            Some(unsafe { rect.assume_init() })
        } else {
            None
        }
    }

    pub fn set_working_perimeter(&mut self, points: &mut [HmdVector2_t]) {
        unsafe {
            self.inner
                .as_mut()
                .SetWorkingPerimeter(points.as_mut_ptr(), points.len() as _)
        }
    }

    pub fn commit_working_copy(&mut self, config: sys::EChaperoneConfigFile) -> bool {
        unsafe { self.inner.as_mut().CommitWorkingCopy(config) }
    }

    pub fn revert_working_copy(&mut self) {
        unsafe { self.inner.as_mut().RevertWorkingCopy() }
    }

    pub fn reload_from_disk(&mut self, config: sys::EChaperoneConfigFile) {
        unsafe { self.inner.as_mut().ReloadFromDisk(config) }
    }

    pub fn show_working_set_preview(&mut self) {
        unsafe { self.inner.as_mut().ShowWorkingSetPreview() }
    }

    pub fn hide_working_set_preview(&mut self) {
        unsafe { self.inner.as_mut().HideWorkingSetPreview() }
    }
}
