use crate::*;
use anyhow::{anyhow, Result};

static UNKNOWN_ERROR: &str = "unknown error";

impl ImageBuffer {
    #[inline(always)]
    pub(crate) fn mut_self_or_error(
        &mut self,
        is_ok: bool,
    ) -> Result<&mut Self> {
        if is_ok && self.is_ok() {
            Ok(self)
        } else {
            Err(anyhow!(self.error(true).unwrap_or(UNKNOWN_ERROR.into())))
        }
    }

    #[inline(always)]
    pub(crate) fn self_or_error(self, is_ok: bool) -> Result<Self> {
        if is_ok && self.is_ok() {
            Ok(self)
        } else {
            Err(anyhow!(self.error(true).unwrap_or(UNKNOWN_ERROR.into())))
        }
    }
}
