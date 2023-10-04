use std::io;
use crate::buffer::Type;

use super::Parameters;
use crate::device::MultiPlaneDevice;
use crate::format::FourCC;
use crate::format::{Description as FormatDescription, MultiPlaneFormat};
use crate::frameinterval::FrameInterval;
use crate::framesize::FrameSize;
use crate::video::traits::{Capture, Video, VideoBase};

impl Capture for MultiPlaneDevice {
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
        <Self as VideoBase>::enum_formats(self, Type::VideoCaptureMplane)
    }

    fn format(&self) -> io::Result<MultiPlaneFormat> {
        <Self as Video>::format(self, Type::VideoCaptureMplane)
    }

    fn set_format(&self, fmt: &MultiPlaneFormat) -> io::Result<MultiPlaneFormat> {
        <Self as Video>::set_format(self, Type::VideoCaptureMplane, fmt)
    }

    type Format = MultiPlaneFormat;

    fn params(&self) -> io::Result<Parameters> {
        unimplemented!()
    }

    fn set_params(&self, _params: &Parameters) -> io::Result<Parameters> {
        unimplemented!()
    }
}
