#[doc = "Register `ERRORSTATUS` reader"]
pub type R = crate::R<ErrorstatusSpec>;
#[doc = "Error status when the ERROR event is generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Errorstatus {
    #[doc = "0: No errors have occurred"]
    NoError = 0,
    #[doc = "1: End of INPTR job list before data structure was read."]
    PrematureInptrEnd = 1,
    #[doc = "2: End of OUTPTR job list before data structure was read."]
    PrematureOutptrEnd = 2,
    #[doc = "3: Deprecated enumerator - Encryption aborted due to higher priority peripheral requesting or using the AES module."]
    EncryptionTooSlow = 3,
    #[doc = "3: Encryption aborted due to higher priority peripheral requesting or using the AES module."]
    Aborted = 3,
    #[doc = "4: Bus error during DMA access."]
    DmaError = 4,
}
impl From<Errorstatus> for u8 {
    #[inline(always)]
    fn from(variant: Errorstatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Errorstatus {
    type Ux = u8;
}
impl crate::IsEnum for Errorstatus {}
#[doc = "Field `ERRORSTATUS` reader - Error status when the ERROR event is generated"]
pub type ErrorstatusR = crate::FieldReader<Errorstatus>;
impl ErrorstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Errorstatus> {
        match self.bits {
            0 => Some(Errorstatus::NoError),
            1 => Some(Errorstatus::PrematureInptrEnd),
            2 => Some(Errorstatus::PrematureOutptrEnd),
            3 => Some(Errorstatus::EncryptionTooSlow),
            3 => Some(Errorstatus::Aborted),
            4 => Some(Errorstatus::DmaError),
            _ => None,
        }
    }
    #[doc = "No errors have occurred"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Errorstatus::NoError
    }
    #[doc = "End of INPTR job list before data structure was read."]
    #[inline(always)]
    pub fn is_premature_inptr_end(&self) -> bool {
        *self == Errorstatus::PrematureInptrEnd
    }
    #[doc = "End of OUTPTR job list before data structure was read."]
    #[inline(always)]
    pub fn is_premature_outptr_end(&self) -> bool {
        *self == Errorstatus::PrematureOutptrEnd
    }
    #[doc = "Deprecated enumerator - Encryption aborted due to higher priority peripheral requesting or using the AES module."]
    #[inline(always)]
    pub fn is_encryption_too_slow(&self) -> bool {
        *self == Errorstatus::EncryptionTooSlow
    }
    #[doc = "Encryption aborted due to higher priority peripheral requesting or using the AES module."]
    #[inline(always)]
    pub fn is_aborted(&self) -> bool {
        *self == Errorstatus::Aborted
    }
    #[doc = "Bus error during DMA access."]
    #[inline(always)]
    pub fn is_dma_error(&self) -> bool {
        *self == Errorstatus::DmaError
    }
}
impl R {
    #[doc = "Bits 0:2 - Error status when the ERROR event is generated"]
    #[inline(always)]
    pub fn errorstatus(&self) -> ErrorstatusR {
        ErrorstatusR::new((self.bits & 7) as u8)
    }
}
#[doc = "Error status\n\nYou can [`read`](crate::Reg::read) this register and get [`errorstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorstatusSpec;
impl crate::RegisterSpec for ErrorstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errorstatus::R`](R) reader structure"]
impl crate::Readable for ErrorstatusSpec {}
#[doc = "`reset()` method sets ERRORSTATUS to value 0"]
impl crate::Resettable for ErrorstatusSpec {}
