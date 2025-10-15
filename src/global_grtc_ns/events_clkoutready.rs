#[doc = "Register `EVENTS_CLKOUTREADY` reader"]
pub type R = crate::R<EventsClkoutreadySpec>;
#[doc = "Register `EVENTS_CLKOUTREADY` writer"]
pub type W = crate::W<EventsClkoutreadySpec>;
#[doc = "Event on STATUS.CLKOUT.READY status changed to ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsClkoutready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsClkoutready> for bool {
    #[inline(always)]
    fn from(variant: EventsClkoutready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_CLKOUTREADY` reader - Event on STATUS.CLKOUT.READY status changed to ready"]
pub type EventsClkoutreadyR = crate::BitReader<EventsClkoutready>;
impl EventsClkoutreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsClkoutready {
        match self.bits {
            false => EventsClkoutready::NotGenerated,
            true => EventsClkoutready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsClkoutready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsClkoutready::Generated
    }
}
#[doc = "Field `EVENTS_CLKOUTREADY` writer - Event on STATUS.CLKOUT.READY status changed to ready"]
pub type EventsClkoutreadyW<'a, REG> = crate::BitWriter<'a, REG, EventsClkoutready>;
impl<'a, REG> EventsClkoutreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsClkoutready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsClkoutready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event on STATUS.CLKOUT.READY status changed to ready"]
    #[inline(always)]
    pub fn events_clkoutready(&self) -> EventsClkoutreadyR {
        EventsClkoutreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event on STATUS.CLKOUT.READY status changed to ready"]
    #[inline(always)]
    pub fn events_clkoutready(&mut self) -> EventsClkoutreadyW<'_, EventsClkoutreadySpec> {
        EventsClkoutreadyW::new(self, 0)
    }
}
#[doc = "Event on STATUS.CLKOUT.READY status changed to ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_clkoutready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_clkoutready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsClkoutreadySpec;
impl crate::RegisterSpec for EventsClkoutreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_clkoutready::R`](R) reader structure"]
impl crate::Readable for EventsClkoutreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_clkoutready::W`](W) writer structure"]
impl crate::Writable for EventsClkoutreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_CLKOUTREADY to value 0"]
impl crate::Resettable for EventsClkoutreadySpec {}
