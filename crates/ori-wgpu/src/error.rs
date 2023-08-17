use std::fmt::Display;

use wgpu::{CreateSurfaceError, RequestDeviceError};
use winit::error::OsError;

#[derive(Debug)]
pub enum RenderError {
    CreateSurface(CreateSurfaceError),
    AdapterNotFound,
    RequestDevice(RequestDeviceError),
    SurfaceIncompatible,
}

impl From<CreateSurfaceError> for RenderError {
    fn from(err: CreateSurfaceError) -> Self {
        Self::CreateSurface(err)
    }
}

impl From<RequestDeviceError> for RenderError {
    fn from(err: RequestDeviceError) -> Self {
        Self::RequestDevice(err)
    }
}

impl Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::CreateSurface(err) => write!(f, "Failed to create surface: {}", err),
            RenderError::AdapterNotFound => write!(f, "No adapter found"),
            RenderError::RequestDevice(err) => write!(f, "Failed to request device: {}", err),
            RenderError::SurfaceIncompatible => write!(f, "Surface incompatible with adapter"),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Render(RenderError),
    OsError(OsError),
}

impl From<RenderError> for Error {
    fn from(err: RenderError) -> Self {
        Self::Render(err)
    }
}

impl From<OsError> for Error {
    fn from(err: OsError) -> Self {
        Self::OsError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Render(err) => write!(f, "{}", err),
            Error::OsError(err) => write!(f, "{}", err),
        }
    }
}
