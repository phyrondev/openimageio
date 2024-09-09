use crate::*;
use anyhow::{anyhow, Result};
use log::error;

static UNKNOWN_ERROR: &str = "unknown error";

impl ImageBuffer {
    #[inline(always)]
    pub(crate) fn ok_or_error_owned(self) -> Result<Self> {
        if self.is_ok() {
            Ok(self)
        } else {
            Err(anyhow!(self.error(true).unwrap_or(UNKNOWN_ERROR.into())))
        }
    }

    #[inline(always)]
    pub(crate) fn ok_or_error(&mut self, is_ok: bool) -> Result<&mut Self> {
        if is_ok && self.is_ok() {
            Ok(self)
        } else {
            Err(anyhow!(self.error(true).unwrap_or(UNKNOWN_ERROR.into())))
        }
    }

    #[inline(always)]
    pub(crate) fn self_or_error(self) -> Result<Self> {
        if self.is_ok() {
            Ok(self)
        } else {
            Err(anyhow!(self.error(true).unwrap_or(UNKNOWN_ERROR.into())))
        }
    }

    #[inline(always)]
    pub(crate) fn ok_or_log_error(&mut self, is_ok: bool) -> &mut Self {
        if !is_ok || !self.is_ok() {
            error!("{}", self.error(true).unwrap_or(UNKNOWN_ERROR.into()))
        }
        self
    }
}
