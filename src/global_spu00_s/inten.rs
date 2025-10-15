#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Periphaccerr {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Periphaccerr> for bool {
    #[inline(always)]
    fn from(variant: Periphaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHACCERR` reader - Enable or disable interrupt for event PERIPHACCERR"]
pub type PeriphaccerrR = crate::BitReader<Periphaccerr>;
impl PeriphaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Periphaccerr {
        match self.bits {
            false => Periphaccerr::Disabled,
            true => Periphaccerr::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Periphaccerr::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Periphaccerr::Enabled
    }
}
#[doc = "Field `PERIPHACCERR` writer - Enable or disable interrupt for event PERIPHACCERR"]
pub type PeriphaccerrW<'a, REG> = crate::BitWriter<'a, REG, Periphaccerr>;
impl<'a, REG> PeriphaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Periphaccerr::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Periphaccerr::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&self) -> PeriphaccerrR {
        PeriphaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&mut self) -> PeriphaccerrW<'_, IntenSpec> {
        PeriphaccerrW::new(self, 0)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
