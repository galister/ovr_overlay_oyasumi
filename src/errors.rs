use crate::sys;

use derive_more::{From, Into};
// use std::ffi::CStr;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq)]
pub struct EVRInitError(sys::EVRInitError);
impl EVRInitError {
    pub fn new(err: sys::EVRInitError) -> Result<(), Self> {
        if err == sys::EVRInitError::VRInitError_None {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    // pub fn description(&self) -> &'static str {
    //     let desc: &'static CStr =
    //         unsafe { CStr::from_ptr(sys::VR_GetVRInitErrorAsSymbol(self.0.clone())) };
    //     desc.to_str().unwrap()
    // }

    pub fn inner(&self) -> sys::EVRInitError {
        self.0.clone()
    }
}
impl Display for EVRInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone() as u8;
        // let desc = self.description();
        // write!(f, "EVRInitError({num})`: {desc}`")
        write!(f, "EVRInitError({num})")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct EVROverlayError(sys::EVROverlayError);
impl EVROverlayError {
    pub fn new(err: sys::EVROverlayError) -> Result<(), Self> {
        if err == sys::EVROverlayError::VROverlayError_None {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    pub fn description(&self) -> &'static str {
        use sys::EVROverlayError::*;
        match self.0 {
            VROverlayError_None => "None",
            VROverlayError_UnknownOverlay => "UnknownOverlay",
            VROverlayError_InvalidHandle => "InvalidHandle",
            VROverlayError_PermissionDenied => "PermissionDenied",
            VROverlayError_OverlayLimitExceeded => "OverlayLimitExceeded",
            VROverlayError_WrongVisibilityType => "WrongVisibilityType",
            VROverlayError_KeyTooLong => "KeyTooLong",
            VROverlayError_NameTooLong => "NameTooLong",
            VROverlayError_KeyInUse => "KeyInUse",
            VROverlayError_WrongTransformType => "WrongTransformType",
            VROverlayError_InvalidTrackedDevice => "InvalidTrackedDevice",
            VROverlayError_InvalidParameter => "InvalidParameter",
            VROverlayError_ThumbnailCantBeDestroyed => "ThumbnailCantBeDestroyed",
            VROverlayError_ArrayTooSmall => "ArrayTooSmall",
            VROverlayError_RequestFailed => "RequestFailed",
            VROverlayError_InvalidTexture => "InvalidTexture",
            VROverlayError_UnableToLoadFile => "UnableToLoadFile",
            VROverlayError_KeyboardAlreadyInUse => "KeyboardAlreadyInUse",
            VROverlayError_NoNeighbor => "NoNeighbor",
            VROverlayError_TooManyMaskPrimitives => "TooManyMaskPrimitives",
            VROverlayError_BadMaskPrimitive => "BadMaskPrimitive",
            VROverlayError_TextureAlreadyLocked => "TextureAlreadyLocked",
            VROverlayError_TextureNotLocked => "TextureNotLocked",
            VROverlayError_TextureLockCapacityReached => "TextureLockCapacityReached",
            VROverlayError_TimedOut => "TimedOut",
        }
    }

    pub fn inner(&self) -> sys::EVROverlayError {
        self.0.clone()
    }
}
impl Display for EVROverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone() as u8;
        let desc = self.description();
        write!(f, "EVROverlayError({num}): {desc}")
    }
}

#[cfg(feature = "ovr_system")]
#[derive(Into, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ETrackedPropertyError(sys::ETrackedPropertyError);

#[cfg(feature = "ovr_system")]
impl ETrackedPropertyError {
    pub fn new(err: sys::ETrackedPropertyError) -> Result<(), Self> {
        if err == sys::ETrackedPropertyError::TrackedProp_Success {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    pub fn description(&self) -> &'static str {
        use sys::ETrackedPropertyError::*;
        match self.0 {
            TrackedProp_Success => "Success",
            TrackedProp_WrongDataType => "WrongDataType",
            TrackedProp_WrongDeviceClass => "WrongDeviceClass",
            TrackedProp_BufferTooSmall => "BufferTooSmall",
            TrackedProp_UnknownProperty => "UnknownProperty",
            TrackedProp_InvalidDevice => "InvalidDevice",
            TrackedProp_CouldNotContactServer => "CouldNotContactServer",
            TrackedProp_ValueNotProvidedByDevice => "ValueNotProvidedByDevice",
            TrackedProp_StringExceedsMaximumLength => "StringExceedsMaximumLength",
            TrackedProp_NotYetAvailable => "NotYetAvailable",
            TrackedProp_PermissionDenied => "PermissionDenied",
            TrackedProp_InvalidOperation => "InvalidOperation",
            TrackedProp_CannotWriteToWildcards => "CannotWriteToWildcards",
            TrackedProp_IPCReadFailure => "IPCReadFailure",
            TrackedProp_OutOfMemory => "OutOfMemory",
            TrackedProp_InvalidContainer => "InvalidContainer",
        }
    }

    pub fn inner(&self) -> sys::ETrackedPropertyError {
        self.0.clone()
    }
}

#[cfg(feature = "ovr_system")]
impl Display for ETrackedPropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone() as u8;
        let desc = self.description();
        write!(f, "ETrackedPropertyError({num}): {desc}")
    }
}
#[cfg(feature = "ovr_input")]
#[derive(From, Into, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVRInputError(sys::EVRInputError);

#[cfg(feature = "ovr_input")]
impl EVRInputError {
    pub fn new(err: sys::EVRInputError) -> Result<(), Self> {
        if err == sys::EVRInputError::VRInputError_None {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    pub fn description(&self) -> &'static str {
        use sys::EVRInputError::*;
        match self.0 {
            VRInputError_None => "None",
            VRInputError_NameNotFound => "NameNotFound",
            VRInputError_WrongType => "WrongType",
            VRInputError_InvalidHandle => "InvalidHandle",
            VRInputError_InvalidParam => "InvalidParam",
            VRInputError_NoSteam => "NoSteam",
            VRInputError_MaxCapacityReached => "MaxCapacityReached",
            VRInputError_IPCError => "IPCError",
            VRInputError_NoActiveActionSet => "NoActiveActionSet",
            VRInputError_InvalidDevice => "InvalidDevice",
            VRInputError_InvalidSkeleton => "InvalidSkeleton",
            VRInputError_InvalidBoneCount => "InvalidBoneCount",
            VRInputError_InvalidCompressedData => "InvalidCompressedData",
            VRInputError_NoData => "NoData",
            VRInputError_BufferTooSmall => "BufferTooSmall",
            VRInputError_MismatchedActionManifest => "MismatchedActionManifest",
            VRInputError_MissingSkeletonData => "MissingSkeletonData",
            VRInputError_InvalidBoneIndex => "InvalidBoneIndex",
            VRInputError_InvalidPriority => "InvalidPriority",
            VRInputError_PermissionDenied => "PermissionDenied",
            VRInputError_InvalidRenderModel => "InvalidRenderModel",
        }
    }

    pub fn inner(&self) -> sys::EVRInputError {
        self.0.clone()
    }
}

#[cfg(feature = "ovr_input")]
impl Display for EVRInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone() as u8;
        let desc = self.description();
        write!(f, "EVRInputError({num}): {desc}")
    }
}

#[cfg(feature = "ovr_compositor")]
#[derive(Into, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVRCompositorError(sys::EVRCompositorError);

#[cfg(feature = "ovr_compositor")]
impl EVRCompositorError {
    pub fn new(err: sys::EVRCompositorError) -> Result<(), Self> {
        if err == sys::EVRCompositorError::VRCompositorError_None {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    pub fn description(&self) -> &'static str {
        use sys::EVRCompositorError::*;
        match self.0 {
            VRCompositorError_None => "None",
            VRCompositorError_RequestFailed => "RequestFailed",
            VRCompositorError_IncompatibleVersion => "IncompatibleVersion",
            VRCompositorError_DoNotHaveFocus => "DoNotHaveFocus",
            VRCompositorError_InvalidTexture => "InvalidTexture",
            VRCompositorError_IsNotSceneApplication => "IsNotSceneApplication",
            VRCompositorError_TextureIsOnWrongDevice => "TextureIsOnWrongDevice",
            VRCompositorError_TextureUsesUnsupportedFormat => "TextureUsesUnsupportedFormat",
            VRCompositorError_SharedTexturesNotSupported => "SharedTexturesNotSupported",
            VRCompositorError_IndexOutOfRange => "IndexOutOfRange",
            VRCompositorError_AlreadySubmitted => "AlreadySubmitted",
            VRCompositorError_AlreadySet => "AlreadySet",
            VRCompositorError_InvalidBounds => "InvalidBounds",
        }
    }

    pub fn inner(&self) -> sys::EVRCompositorError {
        self.0.clone()
    }
}

#[cfg(feature = "ovr_applications")]
#[derive(Into, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVRApplicationError(sys::EVRApplicationError);

#[cfg(feature = "ovr_applications")]
impl EVRApplicationError {
    pub fn new(err: sys::EVRApplicationError) -> Result<(), Self> {
        if err == sys::EVRApplicationError::VRApplicationError_None {
            Ok(())
        } else {
            Err(Self(err))
        }
    }

    pub fn description(&self) -> &'static str {
        use sys::EVRApplicationError::*;
        match self.0 {
            VRApplicationError_None => "None",
            VRApplicationError_AppKeyAlreadyExists => "AppKeyAlreadyExists",
            VRApplicationError_NoManifest => "NoManifest",
            VRApplicationError_NoApplication => "NoApplication",
            VRApplicationError_InvalidIndex => "InvalidIndex",
            VRApplicationError_UnknownApplication => "UnknownApplication",
            VRApplicationError_IPCFailed => "IPCFailed",
            VRApplicationError_ApplicationAlreadyRunning => "ApplicationAlreadyRunning",
            VRApplicationError_InvalidManifest => "InvalidManifest",
            VRApplicationError_InvalidApplication => "InvalidApplication",
            VRApplicationError_LaunchFailed => "LaunchFailed",
            VRApplicationError_ApplicationAlreadyStarting => "ApplicationAlreadyStarting",
            VRApplicationError_LaunchInProgress => "LaunchInProgress",
            VRApplicationError_OldApplicationQuitting => "OldApplicationQuitting",
            VRApplicationError_TransitionAborted => "TransitionAborted",
            VRApplicationError_IsTemplate => "IsTemplate",
            VRApplicationError_SteamVRIsExiting => "SteamVRIsExiting",
            VRApplicationError_BufferTooSmall => "BufferTooSmall",
            VRApplicationError_PropertyNotSet => "PropertyNotSet",
            VRApplicationError_UnknownProperty => "UnknownProperty",
            VRApplicationError_InvalidParameter => "InvalidParameter",
            VRApplicationError_NotImplemented => "NotImplemented",
        }
    }

    pub fn inner(&self) -> sys::EVRApplicationError {
        self.0.clone()
    }
}

#[cfg(feature = "ovr_applications")]
impl Display for EVRApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = self.0.clone() as u8;
        let desc = self.description();
        write!(f, "EVRApplicationError({num}): {desc}")
    }
}

#[derive(From)]
pub enum InitError {
    AlreadyInitialized,
    Sys(EVRInitError),
}
