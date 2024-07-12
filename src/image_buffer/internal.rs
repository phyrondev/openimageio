use crate::*;
use anyhow::{anyhow, Result};
use log::error;

static UNKNOWN_ERROR: &str = "unknown error";

impl<'a> ImageBuffer<'a> {
    #[inline(always)]
    pub(crate) fn ok_or_error(&mut self, is_ok: bool) -> Result<&mut Self> {
        if !is_ok || self.is_error() {
            Err(anyhow!(self
                .error(Some(true))
                .unwrap_or(UNKNOWN_ERROR.into())))
        } else {
            Ok(self)
        }
    }

    #[inline(always)]
    pub(crate) fn ok_or_log_error(&mut self, is_ok: bool) -> &mut Self {
        if !is_ok || self.is_error() {
            error!("{}", self.error(Some(true)).unwrap_or(UNKNOWN_ERROR.into()))
        }
        self
    }
}
