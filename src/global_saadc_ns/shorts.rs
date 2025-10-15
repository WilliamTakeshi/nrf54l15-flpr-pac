#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event DONE and task SAMPLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoneSample {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<DoneSample> for bool {
    #[inline(always)]
    fn from(variant: DoneSample) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE_SAMPLE` reader - Shortcut between event DONE and task SAMPLE"]
pub type DoneSampleR = crate::BitReader<DoneSample>;
impl DoneSampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DoneSample {
        match self.bits {
            false => DoneSample::Disabled,
            true => DoneSample::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DoneSample::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DoneSample::Enabled
    }
}
#[doc = "Field `DONE_SAMPLE` writer - Shortcut between event DONE and task SAMPLE"]
pub type DoneSampleW<'a, REG> = crate::BitWriter<'a, REG, DoneSample>;
impl<'a, REG> DoneSampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DoneSample::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DoneSample::Enabled)
    }
}
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndStart {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndStart> for bool {
    #[inline(always)]
    fn from(variant: EndStart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub type EndStartR = crate::BitReader<EndStart>;
impl EndStartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndStart {
        match self.bits {
            false => EndStart::Disabled,
            true => EndStart::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndStart::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndStart::Enabled
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub type EndStartW<'a, REG> = crate::BitWriter<'a, REG, EndStart>;
impl<'a, REG> EndStartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndStart::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event DONE and task SAMPLE"]
    #[inline(always)]
    pub fn done_sample(&self) -> DoneSampleR {
        DoneSampleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> EndStartR {
        EndStartR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event DONE and task SAMPLE"]
    #[inline(always)]
    pub fn done_sample(&mut self) -> DoneSampleW<'_, ShortsSpec> {
        DoneSampleW::new(self, 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&mut self) -> EndStartW<'_, ShortsSpec> {
        EndStartW::new(self, 1)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {}
