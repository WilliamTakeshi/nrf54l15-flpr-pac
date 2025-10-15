#[doc = "Register `EVENTS_PWMREADY` reader"]
pub type R = crate::R<EventsPwmreadySpec>;
#[doc = "Register `EVENTS_PWMREADY` writer"]
pub type W = crate::W<EventsPwmreadySpec>;
#[doc = "Event on STATUS.PWM.READY status changed to ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsPwmready {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsPwmready> for bool {
    #[inline(always)]
    fn from(variant: EventsPwmready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_PWMREADY` reader - Event on STATUS.PWM.READY status changed to ready"]
pub type EventsPwmreadyR = crate::BitReader<EventsPwmready>;
impl EventsPwmreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsPwmready {
        match self.bits {
            false => EventsPwmready::NotGenerated,
            true => EventsPwmready::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsPwmready::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsPwmready::Generated
    }
}
#[doc = "Field `EVENTS_PWMREADY` writer - Event on STATUS.PWM.READY status changed to ready"]
pub type EventsPwmreadyW<'a, REG> = crate::BitWriter<'a, REG, EventsPwmready>;
impl<'a, REG> EventsPwmreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPwmready::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsPwmready::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event on STATUS.PWM.READY status changed to ready"]
    #[inline(always)]
    pub fn events_pwmready(&self) -> EventsPwmreadyR {
        EventsPwmreadyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event on STATUS.PWM.READY status changed to ready"]
    #[inline(always)]
    pub fn events_pwmready(&mut self) -> EventsPwmreadyW<'_, EventsPwmreadySpec> {
        EventsPwmreadyW::new(self, 0)
    }
}
#[doc = "Event on STATUS.PWM.READY status changed to ready\n\nYou can [`read`](crate::Reg::read) this register and get [`events_pwmready::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_pwmready::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsPwmreadySpec;
impl crate::RegisterSpec for EventsPwmreadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_pwmready::R`](R) reader structure"]
impl crate::Readable for EventsPwmreadySpec {}
#[doc = "`write(|w| ..)` method takes [`events_pwmready::W`](W) writer structure"]
impl crate::Writable for EventsPwmreadySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_PWMREADY to value 0"]
impl crate::Resettable for EventsPwmreadySpec {}
