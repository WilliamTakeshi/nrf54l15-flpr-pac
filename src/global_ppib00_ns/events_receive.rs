#[doc = "Register `EVENTS_RECEIVE[%s]` reader"]
pub type R = crate::R<EventsReceiveSpec>;
#[doc = "Register `EVENTS_RECEIVE[%s]` writer"]
pub type W = crate::W<EventsReceiveSpec>;
#[doc = "This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsReceive {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsReceive> for bool {
    #[inline(always)]
    fn from(variant: EventsReceive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RECEIVE` reader - This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event."]
pub type EventsReceiveR = crate::BitReader<EventsReceive>;
impl EventsReceiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsReceive {
        match self.bits {
            false => EventsReceive::NotGenerated,
            true => EventsReceive::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsReceive::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsReceive::Generated
    }
}
#[doc = "Field `EVENTS_RECEIVE` writer - This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event."]
pub type EventsReceiveW<'a, REG> = crate::BitWriter<'a, REG, EventsReceive>;
impl<'a, REG> EventsReceiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReceive::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReceive::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event."]
    #[inline(always)]
    pub fn events_receive(&self) -> EventsReceiveR {
        EventsReceiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event."]
    #[inline(always)]
    pub fn events_receive(&mut self) -> EventsReceiveW<'_, EventsReceiveSpec> {
        EventsReceiveW::new(self, 0)
    }
}
#[doc = "Description collection: This event is unused, but the PPIB provides the PUBLISH event to connect RECEIVE \\[n\\] event.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_receive::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_receive::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReceiveSpec;
impl crate::RegisterSpec for EventsReceiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_receive::R`](R) reader structure"]
impl crate::Readable for EventsReceiveSpec {}
#[doc = "`write(|w| ..)` method takes [`events_receive::W`](W) writer structure"]
impl crate::Writable for EventsReceiveSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVENTS_RECEIVE[%s] to value 0"]
impl crate::Resettable for EventsReceiveSpec {}
