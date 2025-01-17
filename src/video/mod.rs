use std::{convert::TryFrom, io, mem, os::fd::AsRawFd};

use crate::v4l_sys::*;
use crate::{
    buffer, format::Description as FormatDescription, v4l2, Device, Format, FourCC, FrameInterval,
    FrameSize,
};

pub mod traits;

pub mod capture;
pub mod output;

pub use traits::{Capture, Output};
use crate::device::{MultiPlaneDevice, PlanarDevice};
use crate::format::MultiPlaneFormat;

impl<const M: bool> traits::VideoBase for PlanarDevice<M> {
    fn enum_frameintervals(
        &self,
        fourcc: FourCC,
        width: u32,
        height: u32,
    ) -> io::Result<Vec<FrameInterval>> {
        let mut frameintervals = Vec::new();
        let mut v4l2_struct = v4l2_frmivalenum {
            index: 0,
            pixel_format: fourcc.into(),
            width,
            height,
            ..unsafe { mem::zeroed() }
        };

        loop {
            let ret = unsafe {
                v4l2::ioctl(
                    self.handle().as_raw_fd(),
                    v4l2::vidioc::VIDIOC_ENUM_FRAMEINTERVALS,
                    &mut v4l2_struct as *mut _ as *mut std::os::raw::c_void,
                )
            };

            if ret.is_err() {
                if v4l2_struct.index == 0 {
                    return Err(ret.err().unwrap());
                } else {
                    return Ok(frameintervals);
                }
            }

            if let Ok(frame_interval) = FrameInterval::try_from(v4l2_struct) {
                frameintervals.push(frame_interval);
            }

            v4l2_struct.index += 1;
        }
    }

    fn enum_framesizes(&self, fourcc: FourCC) -> io::Result<Vec<FrameSize>> {
        let mut framesizes = Vec::new();
        let mut v4l2_struct = v4l2_frmsizeenum {
            index: 0,
            pixel_format: fourcc.into(),
            ..unsafe { mem::zeroed() }
        };

        loop {
            let ret = unsafe {
                v4l2::ioctl(
                    self.handle().as_raw_fd(),
                    v4l2::vidioc::VIDIOC_ENUM_FRAMESIZES,
                    &mut v4l2_struct as *mut _ as *mut std::os::raw::c_void,
                )
            };

            if ret.is_err() {
                if v4l2_struct.index == 0 {
                    return Err(ret.err().unwrap());
                } else {
                    return Ok(framesizes);
                }
            }

            if let Ok(frame_size) = FrameSize::try_from(v4l2_struct) {
                framesizes.push(frame_size);
            }

            v4l2_struct.index += 1;
        }
    }

    fn enum_formats(&self, typ: buffer::Type) -> io::Result<Vec<FormatDescription>> {
        let mut formats: Vec<FormatDescription> = Vec::new();
        let mut v4l2_fmt = v4l2_fmtdesc {
            index: 0,
            type_: typ as u32,
            ..unsafe { mem::zeroed() }
        };

        let mut ret: io::Result<()>;

        unsafe {
            ret = v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_ENUM_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            );
        }

        if ret.is_err() {
            // Enumerating the first format (at index 0) failed, so there are no formats available
            // for this device. Just return an empty vec in this case.
            return Ok(Vec::new());
        }

        while ret.is_ok() {
            formats.push(FormatDescription::from(v4l2_fmt));
            v4l2_fmt.index += 1;

            unsafe {
                v4l2_fmt.description = mem::zeroed();
            }

            unsafe {
                ret = v4l2::ioctl(
                    self.handle().as_raw_fd(),
                    v4l2::vidioc::VIDIOC_ENUM_FMT,
                    &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
                );
            }
        }

        Ok(formats)
    }
}

impl traits::Video for Device {
    type Format = Format;

    fn format(&self, typ: buffer::Type) -> io::Result<Format> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: typ as u32,
                ..mem::zeroed()
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_G_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;

            Ok(Format::from(v4l2_fmt.fmt.pix))
        }
    }

    fn set_format(&self, typ: buffer::Type, fmt: &Format) -> io::Result<Format> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: typ as u32,
                fmt: v4l2_format__bindgen_ty_1 { pix: (*fmt).into() },
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_S_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;
        }

        <Self as traits::Video>::format(self, typ)
    }
}


impl traits::Video for MultiPlaneDevice {
    type Format = MultiPlaneFormat;

    fn format(&self, typ: buffer::Type) -> io::Result<MultiPlaneFormat> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: typ as u32,
                ..mem::zeroed()
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_G_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;

            Ok(MultiPlaneFormat::from(v4l2_fmt.fmt.pix_mp))
        }
    }

    fn set_format(&self, typ: buffer::Type, fmt: &MultiPlaneFormat) -> io::Result<MultiPlaneFormat> {
        unsafe {
            let mut v4l2_fmt = v4l2_format {
                type_: typ as u32,
                fmt: v4l2_format__bindgen_ty_1 { pix_mp: fmt.clone().into() },
            };
            v4l2::ioctl(
                self.handle().as_raw_fd(),
                v4l2::vidioc::VIDIOC_S_FMT,
                &mut v4l2_fmt as *mut _ as *mut std::os::raw::c_void,
            )?;
        }

        <Self as traits::Video>::format(self, typ)
    }
}
