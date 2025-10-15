#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `FETCHBUSY` reader - This bit is high as long as the fetcher DMA is busy."]
pub type FetchbusyR = crate::BitReader;
#[doc = "Field `PUSHBUSY` reader - This bit is high as long as the pusher DMA is busy."]
pub type PushbusyR = crate::BitReader;
#[doc = "Field `FETCHNOTEMPTY` reader - Not empty flag for fetcher DMA input FIFO"]
pub type FetchnotemptyR = crate::BitReader;
#[doc = "Field `PUSHWAITINGFIFO` reader - Pusher DMA Waiting FIFO. This bit is high when the pusher is waiting for more data in output FIFO."]
pub type PushwaitingfifoR = crate::BitReader;
#[doc = "Field `SOFTRSTBUSY` reader - This bit is high when the soft reset is on going"]
pub type SoftrstbusyR = crate::BitReader;
#[doc = "Field `PUSHNBDATA` reader - Amount of data in the pusher DMA output FIFO"]
pub type PushnbdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - This bit is high as long as the fetcher DMA is busy."]
    #[inline(always)]
    pub fn fetchbusy(&self) -> FetchbusyR {
        FetchbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is high as long as the pusher DMA is busy."]
    #[inline(always)]
    pub fn pushbusy(&self) -> PushbusyR {
        PushbusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Not empty flag for fetcher DMA input FIFO"]
    #[inline(always)]
    pub fn fetchnotempty(&self) -> FetchnotemptyR {
        FetchnotemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pusher DMA Waiting FIFO. This bit is high when the pusher is waiting for more data in output FIFO."]
    #[inline(always)]
    pub fn pushwaitingfifo(&self) -> PushwaitingfifoR {
        PushwaitingfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is high when the soft reset is on going"]
    #[inline(always)]
    pub fn softrstbusy(&self) -> SoftrstbusyR {
        SoftrstbusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Amount of data in the pusher DMA output FIFO"]
    #[inline(always)]
    pub fn pushnbdata(&self) -> PushnbdataR {
        PushnbdataR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
