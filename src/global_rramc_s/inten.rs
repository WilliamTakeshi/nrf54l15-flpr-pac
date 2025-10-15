#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event WOKENUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wokenup {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Wokenup> for bool {
    #[inline(always)]
    fn from(variant: Wokenup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WOKENUP` reader - Enable or disable interrupt for event WOKENUP"]
pub type WokenupR = crate::BitReader<Wokenup>;
impl WokenupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wokenup {
        match self.bits {
            false => Wokenup::Disabled,
            true => Wokenup::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wokenup::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wokenup::Enabled
    }
}
#[doc = "Field `WOKENUP` writer - Enable or disable interrupt for event WOKENUP"]
pub type WokenupW<'a, REG> = crate::BitWriter<'a, REG, Wokenup>;
impl<'a, REG> WokenupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wokenup::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wokenup::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Enable or disable interrupt for event READY"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Disabled,
            true => Ready::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ready::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ready::Enabled
    }
}
#[doc = "Field `READY` writer - Enable or disable interrupt for event READY"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG, Ready>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event READYNEXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Readynext {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Readynext> for bool {
    #[inline(always)]
    fn from(variant: Readynext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READYNEXT` reader - Enable or disable interrupt for event READYNEXT"]
pub type ReadynextR = crate::BitReader<Readynext>;
impl ReadynextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Readynext {
        match self.bits {
            false => Readynext::Disabled,
            true => Readynext::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Readynext::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Readynext::Enabled
    }
}
#[doc = "Field `READYNEXT` writer - Enable or disable interrupt for event READYNEXT"]
pub type ReadynextW<'a, REG> = crate::BitWriter<'a, REG, Readynext>;
impl<'a, REG> ReadynextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Readynext::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Readynext::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ACCESSERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accesserror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Accesserror> for bool {
    #[inline(always)]
    fn from(variant: Accesserror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCESSERROR` reader - Enable or disable interrupt for event ACCESSERROR"]
pub type AccesserrorR = crate::BitReader<Accesserror>;
impl AccesserrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accesserror {
        match self.bits {
            false => Accesserror::Disabled,
            true => Accesserror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Accesserror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Accesserror::Enabled
    }
}
#[doc = "Field `ACCESSERROR` writer - Enable or disable interrupt for event ACCESSERROR"]
pub type AccesserrorW<'a, REG> = crate::BitWriter<'a, REG, Accesserror>;
impl<'a, REG> AccesserrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Accesserror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Accesserror::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event ECCERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccerror {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Eccerror> for bool {
    #[inline(always)]
    fn from(variant: Eccerror) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCERROR` reader - Enable or disable interrupt for event ECCERROR"]
pub type EccerrorR = crate::BitReader<Eccerror>;
impl EccerrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccerror {
        match self.bits {
            false => Eccerror::Disabled,
            true => Eccerror::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eccerror::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eccerror::Enabled
    }
}
#[doc = "Field `ECCERROR` writer - Enable or disable interrupt for event ECCERROR"]
pub type EccerrorW<'a, REG> = crate::BitWriter<'a, REG, Eccerror>;
impl<'a, REG> EccerrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eccerror::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Eccerror::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event WOKENUP"]
    #[inline(always)]
    pub fn wokenup(&self) -> WokenupR {
        WokenupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event READYNEXT"]
    #[inline(always)]
    pub fn readynext(&self) -> ReadynextR {
        ReadynextR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event ACCESSERROR"]
    #[inline(always)]
    pub fn accesserror(&self) -> AccesserrorR {
        AccesserrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ECCERROR"]
    #[inline(always)]
    pub fn eccerror(&self) -> EccerrorR {
        EccerrorR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event WOKENUP"]
    #[inline(always)]
    pub fn wokenup(&mut self) -> WokenupW<'_, IntenSpec> {
        WokenupW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntenSpec> {
        ReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event READYNEXT"]
    #[inline(always)]
    pub fn readynext(&mut self) -> ReadynextW<'_, IntenSpec> {
        ReadynextW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event ACCESSERROR"]
    #[inline(always)]
    pub fn accesserror(&mut self) -> AccesserrorW<'_, IntenSpec> {
        AccesserrorW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event ECCERROR"]
    #[inline(always)]
    pub fn eccerror(&mut self) -> EccerrorW<'_, IntenSpec> {
        EccerrorW::new(self, 4)
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
