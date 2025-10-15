#[doc = "Register `STOP` writer"]
pub type W = crate::W<StopSpec>;
#[doc = "Stops operation using easyDMA. This does not trigger an END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - Stops operation using easyDMA. This does not trigger an END event."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stops operation using easyDMA. This does not trigger an END event."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, StopSpec> {
        StopW::new(self, 0)
    }
}
#[doc = "Description cluster: Stops operation using easyDMA. This does not trigger an END event.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopSpec;
impl crate::RegisterSpec for StopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`stop::W`](W) writer structure"]
impl crate::Writable for StopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for StopSpec {}
