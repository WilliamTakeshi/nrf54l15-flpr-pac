#[doc = "Register `INTSTATRAW` reader"]
pub type R = crate::R<IntstatrawSpec>;
#[doc = "Register `INTSTATRAW` writer"]
pub type W = crate::W<IntstatrawSpec>;
#[doc = "Field `FETCHERBLOCKEND` reader - Fetcher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
pub type FetcherblockendR = crate::BitReader;
#[doc = "Field `FETCHERBLOCKEND` writer - Fetcher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
pub type FetcherblockendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHERSTOPPED` reader - Fetcher DMA reached the end of a block with Stop=1, or end of direct transfer"]
pub type FetcherstoppedR = crate::BitReader;
#[doc = "Field `FETCHERSTOPPED` writer - Fetcher DMA reached the end of a block with Stop=1, or end of direct transfer"]
pub type FetcherstoppedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCHERERROR` reader - Bus error during fetcher DMA access"]
pub type FetchererrorR = crate::BitReader;
#[doc = "Field `FETCHERERROR` writer - Bus error during fetcher DMA access"]
pub type FetchererrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERBLOCKEND` reader - Pusher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
pub type PusherblockendR = crate::BitReader;
#[doc = "Field `PUSHERBLOCKEND` writer - Pusher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
pub type PusherblockendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERSTOPPED` reader - Pusher DMA reached the end of a block with Stop=1, or end of direct transfer"]
pub type PusherstoppedR = crate::BitReader;
#[doc = "Field `PUSHERSTOPPED` writer - Pusher DMA reached the end of a block with Stop=1, or end of direct transfer"]
pub type PusherstoppedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUSHERERROR` reader - Bus error during pusher DMA access"]
pub type PushererrorR = crate::BitReader;
#[doc = "Field `PUSHERERROR` writer - Bus error during pusher DMA access"]
pub type PushererrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fetcher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
    #[inline(always)]
    pub fn fetcherblockend(&self) -> FetcherblockendR {
        FetcherblockendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fetcher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn fetcherstopped(&self) -> FetcherstoppedR {
        FetcherstoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus error during fetcher DMA access"]
    #[inline(always)]
    pub fn fetchererror(&self) -> FetchererrorR {
        FetchererrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pusher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
    #[inline(always)]
    pub fn pusherblockend(&self) -> PusherblockendR {
        PusherblockendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pusher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn pusherstopped(&self) -> PusherstoppedR {
        PusherstoppedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus error during pusher DMA access"]
    #[inline(always)]
    pub fn pushererror(&self) -> PushererrorR {
        PushererrorR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fetcher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
    #[inline(always)]
    pub fn fetcherblockend(&mut self) -> FetcherblockendW<'_, IntstatrawSpec> {
        FetcherblockendW::new(self, 0)
    }
    #[doc = "Bit 1 - Fetcher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn fetcherstopped(&mut self) -> FetcherstoppedW<'_, IntstatrawSpec> {
        FetcherstoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus error during fetcher DMA access"]
    #[inline(always)]
    pub fn fetchererror(&mut self) -> FetchererrorW<'_, IntstatrawSpec> {
        FetchererrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Pusher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
    #[inline(always)]
    pub fn pusherblockend(&mut self) -> PusherblockendW<'_, IntstatrawSpec> {
        PusherblockendW::new(self, 3)
    }
    #[doc = "Bit 4 - Pusher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn pusherstopped(&mut self) -> PusherstoppedW<'_, IntstatrawSpec> {
        PusherstoppedW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus error during pusher DMA access"]
    #[inline(always)]
    pub fn pushererror(&mut self) -> PushererrorW<'_, IntstatrawSpec> {
        PushererrorW::new(self, 5)
    }
}
#[doc = "Interrupt Status Raw\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatraw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatraw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatrawSpec;
impl crate::RegisterSpec for IntstatrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatraw::R`](R) reader structure"]
impl crate::Readable for IntstatrawSpec {}
#[doc = "`write(|w| ..)` method takes [`intstatraw::W`](W) writer structure"]
impl crate::Writable for IntstatrawSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTATRAW to value 0"]
impl crate::Resettable for IntstatrawSpec {}
