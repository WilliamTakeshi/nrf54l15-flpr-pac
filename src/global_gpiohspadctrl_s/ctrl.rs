#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RXDELAY` reader - Delay selection"]
pub type RxdelayR = crate::FieldReader;
#[doc = "Field `RXDELAY` writer - Delay selection"]
pub type RxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Enable SCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scken {
    #[doc = "0: Delay chain is reset and delayed sampling is disabled"]
    Disabled = 0,
    #[doc = "1: Delay chain and delayed sampling is active"]
    Enabled = 1,
}
impl From<Scken> for bool {
    #[inline(always)]
    fn from(variant: Scken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKEN` reader - Enable SCK"]
pub type SckenR = crate::BitReader<Scken>;
impl SckenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scken {
        match self.bits {
            false => Scken::Disabled,
            true => Scken::Enabled,
        }
    }
    #[doc = "Delay chain is reset and delayed sampling is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Scken::Disabled
    }
    #[doc = "Delay chain and delayed sampling is active"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Scken::Enabled
    }
}
#[doc = "Field `SCKEN` writer - Enable SCK"]
pub type SckenW<'a, REG> = crate::BitWriter<'a, REG, Scken>;
impl<'a, REG> SckenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Delay chain is reset and delayed sampling is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scken::Disabled)
    }
    #[doc = "Delay chain and delayed sampling is active"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Scken::Enabled)
    }
}
#[doc = "SCK phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sckphase {
    #[doc = "0: Invert SCK phase"]
    Inverted = 0,
    #[doc = "1: Non-inverted SCK phase"]
    NonInverted = 1,
}
impl From<Sckphase> for bool {
    #[inline(always)]
    fn from(variant: Sckphase) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCKPHASE` reader - SCK phase"]
pub type SckphaseR = crate::BitReader<Sckphase>;
impl SckphaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sckphase {
        match self.bits {
            false => Sckphase::Inverted,
            true => Sckphase::NonInverted,
        }
    }
    #[doc = "Invert SCK phase"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Sckphase::Inverted
    }
    #[doc = "Non-inverted SCK phase"]
    #[inline(always)]
    pub fn is_non_inverted(&self) -> bool {
        *self == Sckphase::NonInverted
    }
}
#[doc = "Field `SCKPHASE` writer - SCK phase"]
pub type SckphaseW<'a, REG> = crate::BitWriter<'a, REG, Sckphase>;
impl<'a, REG> SckphaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Invert SCK phase"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Sckphase::Inverted)
    }
    #[doc = "Non-inverted SCK phase"]
    #[inline(always)]
    pub fn non_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Sckphase::NonInverted)
    }
}
#[doc = "Enable CSN synchronization of sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csnen {
    #[doc = "0: Delay chain is reset on active edge of CSN"]
    Enabled = 0,
    #[doc = "1: Delay chain is not reset on active edge of CSN"]
    Disabled = 1,
}
impl From<Csnen> for bool {
    #[inline(always)]
    fn from(variant: Csnen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSNEN` reader - Enable CSN synchronization of sampling"]
pub type CsnenR = crate::BitReader<Csnen>;
impl CsnenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csnen {
        match self.bits {
            false => Csnen::Enabled,
            true => Csnen::Disabled,
        }
    }
    #[doc = "Delay chain is reset on active edge of CSN"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Csnen::Enabled
    }
    #[doc = "Delay chain is not reset on active edge of CSN"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Csnen::Disabled
    }
}
#[doc = "Field `CSNEN` writer - Enable CSN synchronization of sampling"]
pub type CsnenW<'a, REG> = crate::BitWriter<'a, REG, Csnen>;
impl<'a, REG> CsnenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Delay chain is reset on active edge of CSN"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Csnen::Enabled)
    }
    #[doc = "Delay chain is not reset on active edge of CSN"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Csnen::Disabled)
    }
}
#[doc = "Enable delayed sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dataenable {
    #[doc = "0: Delayed sampling is disabled"]
    Disabled = 0,
    #[doc = "15: Delayed sampling is enabled"]
    Enabled = 15,
}
impl From<Dataenable> for u8 {
    #[inline(always)]
    fn from(variant: Dataenable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dataenable {
    type Ux = u8;
}
impl crate::IsEnum for Dataenable {}
#[doc = "Field `DATAENABLE` reader - Enable delayed sampling"]
pub type DataenableR = crate::FieldReader<Dataenable>;
impl DataenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dataenable> {
        match self.bits {
            0 => Some(Dataenable::Disabled),
            15 => Some(Dataenable::Enabled),
            _ => None,
        }
    }
    #[doc = "Delayed sampling is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dataenable::Disabled
    }
    #[doc = "Delayed sampling is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dataenable::Enabled
    }
}
#[doc = "Field `DATAENABLE` writer - Enable delayed sampling"]
pub type DataenableW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dataenable>;
impl<'a, REG> DataenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delayed sampling is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dataenable::Disabled)
    }
    #[doc = "Delayed sampling is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dataenable::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Delay selection"]
    #[inline(always)]
    pub fn rxdelay(&self) -> RxdelayR {
        RxdelayR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Enable SCK"]
    #[inline(always)]
    pub fn scken(&self) -> SckenR {
        SckenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCK phase"]
    #[inline(always)]
    pub fn sckphase(&self) -> SckphaseR {
        SckphaseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable CSN synchronization of sampling"]
    #[inline(always)]
    pub fn csnen(&self) -> CsnenR {
        CsnenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Enable delayed sampling"]
    #[inline(always)]
    pub fn dataenable(&self) -> DataenableR {
        DataenableR::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay selection"]
    #[inline(always)]
    pub fn rxdelay(&mut self) -> RxdelayW<'_, CtrlSpec> {
        RxdelayW::new(self, 0)
    }
    #[doc = "Bit 3 - Enable SCK"]
    #[inline(always)]
    pub fn scken(&mut self) -> SckenW<'_, CtrlSpec> {
        SckenW::new(self, 3)
    }
    #[doc = "Bit 4 - SCK phase"]
    #[inline(always)]
    pub fn sckphase(&mut self) -> SckphaseW<'_, CtrlSpec> {
        SckphaseW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable CSN synchronization of sampling"]
    #[inline(always)]
    pub fn csnen(&mut self) -> CsnenW<'_, CtrlSpec> {
        CsnenW::new(self, 5)
    }
    #[doc = "Bits 6:9 - Enable delayed sampling"]
    #[inline(always)]
    pub fn dataenable(&mut self) -> DataenableW<'_, CtrlSpec> {
        DataenableW::new(self, 6)
    }
}
#[doc = "Input sampling and buffering control (used by the VPR coprocessor for emulating a QSPI peripheral)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
