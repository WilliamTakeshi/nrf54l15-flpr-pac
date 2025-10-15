#[doc = "Register `CLKOUT` reader"]
pub type R = crate::R<ClkoutSpec>;
#[doc = "Register `CLKOUT` writer"]
pub type W = crate::W<ClkoutSpec>;
#[doc = "CLKOUT is ready or busy.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Busy"]
    Busy = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - CLKOUT is ready or busy."]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Busy,
            true => Ready::Ready,
        }
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ready::Busy
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Ready::Ready
    }
}
impl R {
    #[doc = "Bit 0 - CLKOUT is ready or busy."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "CLKOUT configuration status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutSpec;
impl crate::RegisterSpec for ClkoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkout::R`](R) reader structure"]
impl crate::Readable for ClkoutSpec {}
#[doc = "`write(|w| ..)` method takes [`clkout::W`](W) writer structure"]
impl crate::Writable for ClkoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKOUT to value 0x01"]
impl crate::Resettable for ClkoutSpec {
    const RESET_VALUE: u32 = 0x01;
}
