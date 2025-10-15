#[doc = "Register `DISABLE` writer"]
pub type W = crate::W<DisableSpec>;
#[doc = "Field `KEY` writer - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - The ERASEALL sequence is initiated if the value of the KEY fields are non-zero and the KEY fields match on both the CPU and debugger sides."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, DisableSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableSpec;
impl crate::RegisterSpec for DisableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`disable::W`](W) writer structure"]
impl crate::Writable for DisableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DisableSpec {}
