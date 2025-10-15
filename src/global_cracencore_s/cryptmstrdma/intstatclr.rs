#[doc = "Register `INTSTATCLR` reader"]
pub type R = crate::R<IntstatclrSpec>;
#[doc = "Register `INTSTATCLR` writer"]
pub type W = crate::W<IntstatclrSpec>;
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
    pub fn fetcherblockend(&mut self) -> FetcherblockendW<'_, IntstatclrSpec> {
        FetcherblockendW::new(self, 0)
    }
    #[doc = "Bit 1 - Fetcher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn fetcherstopped(&mut self) -> FetcherstoppedW<'_, IntstatclrSpec> {
        FetcherstoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus error during fetcher DMA access"]
    #[inline(always)]
    pub fn fetchererror(&mut self) -> FetchererrorW<'_, IntstatclrSpec> {
        FetchererrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Pusher DMA reached the end of a block (if enabled in the descriptor; scatter-gather only)"]
    #[inline(always)]
    pub fn pusherblockend(&mut self) -> PusherblockendW<'_, IntstatclrSpec> {
        PusherblockendW::new(self, 3)
    }
    #[doc = "Bit 4 - Pusher DMA reached the end of a block with Stop=1, or end of direct transfer"]
    #[inline(always)]
    pub fn pusherstopped(&mut self) -> PusherstoppedW<'_, IntstatclrSpec> {
        PusherstoppedW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus error during pusher DMA access"]
    #[inline(always)]
    pub fn pushererror(&mut self) -> PushererrorW<'_, IntstatclrSpec> {
        PushererrorW::new(self, 5)
    }
}
#[doc = "Interrupt Status Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstatclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatclrSpec;
impl crate::RegisterSpec for IntstatclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatclr::R`](R) reader structure"]
impl crate::Readable for IntstatclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intstatclr::W`](W) writer structure"]
impl crate::Writable for IntstatclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTATCLR to value 0"]
impl crate::Resettable for IntstatclrSpec {}
