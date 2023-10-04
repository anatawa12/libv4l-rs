use std::{io, mem, os::fd::AsRawFd};
use crate::buffer::Type;

use super::Parameters;
use crate::device::MultiPlaneDevice;
use crate::format::FourCC;
use crate::format::{Description as FormatDescription, MultiPlaneFormat};
use crate::frameinterval::FrameInterval;
use crate::framesize::FrameSize;
use crate::v4l2;
use crate::v4l_sys::*;
use crate::video::traits::{Output, Video, VideoBase};

impl Output for MultiPlaneDevice {
    fn enum_frameintervals(
        &self,
        fourcc: FourCC,
        width: u32,
        height: u32,
    ) -> io::Result<Vec<FrameInterval>> {
        <Self as VideoBase>::enum_frameintervals(self, fourcc, width, height)
    }

    fn enum_framesizes(&self, fourcc: FourCC) -> io::Result<Vec<FrameSize>> {
        <Self as VideoBase>::enum_framesizes(self, fourcc)
    }

    fn enum_formats(&self) -> io::Result<Vec<FormatDescription>> {
        <Self as VideoBase>::enum_formats(self, Type::VideoOutputMplane)
    }

    fn format(&self) -> io::Result<MultiPlaneFormat> {
        <Self as Video>::format(self, Type::VideoOutputMplane)
    }

    fn set_format(&self, fmt: &MultiPlaneFormat) -> io::Result<MultiPlaneFormat> {
        <Self as Video>::set_format(self, Type::VideoOutputMplane, fmt)
    }

    type Format = MultiPlaneFormat;


    fn params(&self) -> io::Result<Parameters> {
        unsafe {
            let mut v4l2_params = v4l2_streamparm {
                type_: Type::VideoOutputMplane as u32,
                ..mem::zeroed()
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_G_PARM,
                &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
            )?;

            Ok(Parameters::from(v4l2_params.parm.output))
        }
    }

    fn set_params(&self, params: &Parameters) -> io::Result<Parameters> {
        unsafe {
            let mut v4l2_params = v4l2_streamparm {
                type_: Type::VideoOutputMplane as u32,
                parm: v4l2_streamparm__bindgen_ty_1 {
                    output: (*params).into(),
                },
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_S_PARM,
                &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
            )?;
        }

        self.params()
    }
}
