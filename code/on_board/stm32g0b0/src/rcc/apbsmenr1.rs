#[doc = "Register `APBSMENR1` reader"]
pub type R = crate::R<APBSMENR1_SPEC>;
#[doc = "Register `APBSMENR1` writer"]
pub type W = crate::W<APBSMENR1_SPEC>;
#[doc = "Field `TIM3SMEN` reader - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_R = crate::BitReader;
#[doc = "Field `TIM3SMEN` writer - TIM3 timer clock enable during Sleep mode"]
pub type TIM3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4SMEN` reader - TIM4 timer clock enable during Sleep mode"]
pub type TIM4SMEN_R = crate::BitReader;
#[doc = "Field `TIM4SMEN` writer - TIM4 timer clock enable during Sleep mode"]
pub type TIM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6SMEN` reader - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_R = crate::BitReader;
#[doc = "Field `TIM6SMEN` writer - TIM6 timer clock enable during Sleep mode"]
pub type TIM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7SMEN` reader - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_R = crate::BitReader;
#[doc = "Field `TIM7SMEN` writer - TIM7 timer clock enable during Sleep mode"]
pub type TIM7SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART5SMEN` reader - USART5 clock enable"]
pub type USART5SMEN_R = crate::BitReader;
#[doc = "Field `USART5SMEN` writer - USART5 clock enable"]
pub type USART5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6SMEN` reader - USART6 clock enable"]
pub type USART6SMEN_R = crate::BitReader;
#[doc = "Field `USART6SMEN` writer - USART6 clock enable"]
pub type USART6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBSMEN` reader - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_R = crate::BitReader;
#[doc = "Field `RTCAPBSMEN` writer - RTC APB clock enable during Sleep mode"]
pub type RTCAPBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGSMEN` reader - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_R = crate::BitReader;
#[doc = "Field `WWDGSMEN` writer - WWDG clock enable during Sleep mode"]
pub type WWDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSMEN` reader - USB clock enable during Sleep mode"]
pub type USBSMEN_R = crate::BitReader;
#[doc = "Field `USBSMEN` writer - USB clock enable during Sleep mode"]
pub type USBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2SMEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_R = crate::BitReader;
#[doc = "Field `SPI2SMEN` writer - SPI2 clock enable during Sleep mode"]
pub type SPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3SMEN` reader - SPI3 clock enable during Sleep mode"]
pub type SPI3SMEN_R = crate::BitReader;
#[doc = "Field `SPI3SMEN` writer - SPI3 clock enable during Sleep mode"]
pub type SPI3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2SMEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_R = crate::BitReader;
#[doc = "Field `USART2SMEN` writer - USART2 clock enable during Sleep mode"]
pub type USART2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3SMEN` reader - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_R = crate::BitReader;
#[doc = "Field `USART3SMEN` writer - USART3 clock enable during Sleep mode"]
pub type USART3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART4SMEN` reader - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_R = crate::BitReader;
#[doc = "Field `USART4SMEN` writer - USART4 clock enable during Sleep mode"]
pub type USART4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1SMEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_R = crate::BitReader;
#[doc = "Field `I2C1SMEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2C1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2SMEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_R = crate::BitReader;
#[doc = "Field `I2C2SMEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2C2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3SMEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2C3SMEN_R = crate::BitReader;
#[doc = "Field `I2C3SMEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2C3SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSMEN` reader - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_R = crate::BitReader;
#[doc = "Field `DBGSMEN` writer - Debug support clock enable during Sleep mode"]
pub type DBGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSMEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_R = crate::BitReader;
#[doc = "Field `PWRSMEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3smen(&self) -> TIM3SMEN_R {
        TIM3SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4smen(&self) -> TIM4SMEN_R {
        TIM4SMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6smen(&self) -> TIM6SMEN_R {
        TIM6SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7smen(&self) -> TIM7SMEN_R {
        TIM7SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    pub fn usart5smen(&self) -> USART5SMEN_R {
        USART5SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6smen(&self) -> USART6SMEN_R {
        USART6SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdgsmen(&self) -> WWDGSMEN_R {
        WWDGSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2smen(&self) -> USART2SMEN_R {
        USART2SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3smen(&self) -> USART3SMEN_R {
        USART3SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart4smen(&self) -> USART4SMEN_R {
        USART4SMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2smen(&self) -> I2C2SMEN_R {
        I2C2SMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dbgsmen(&self) -> DBGSMEN_R {
        DBGSMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3smen(&mut self) -> TIM3SMEN_W<APBSMENR1_SPEC> {
        TIM3SMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4smen(&mut self) -> TIM4SMEN_W<APBSMENR1_SPEC> {
        TIM4SMEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - TIM6 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim6smen(&mut self) -> TIM6SMEN_W<APBSMENR1_SPEC> {
        TIM6SMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 timer clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim7smen(&mut self) -> TIM7SMEN_W<APBSMENR1_SPEC> {
        TIM7SMEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - USART5 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart5smen(&mut self) -> USART5SMEN_W<APBSMENR1_SPEC> {
        USART5SMEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - USART6 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6smen(&mut self) -> USART6SMEN_W<APBSMENR1_SPEC> {
        USART6SMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<APBSMENR1_SPEC> {
        RTCAPBSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgsmen(&mut self) -> WWDGSMEN_W<APBSMENR1_SPEC> {
        WWDGSMEN_W::new(self, 11)
    }
    #[doc = "Bit 13 - USB clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usbsmen(&mut self) -> USBSMEN_W<APBSMENR1_SPEC> {
        USBSMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W<APBSMENR1_SPEC> {
        SPI2SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<APBSMENR1_SPEC> {
        SPI3SMEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2smen(&mut self) -> USART2SMEN_W<APBSMENR1_SPEC> {
        USART2SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart3smen(&mut self) -> USART3SMEN_W<APBSMENR1_SPEC> {
        USART3SMEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - USART4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart4smen(&mut self) -> USART4SMEN_W<APBSMENR1_SPEC> {
        USART4SMEN_W::new(self, 19)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W<APBSMENR1_SPEC> {
        I2C1SMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2smen(&mut self) -> I2C2SMEN_W<APBSMENR1_SPEC> {
        I2C2SMEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<APBSMENR1_SPEC> {
        I2C3SMEN_W::new(self, 23)
    }
    #[doc = "Bit 27 - Debug support clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsmen(&mut self) -> DBGSMEN_W<APBSMENR1_SPEC> {
        DBGSMEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<APBSMENR1_SPEC> {
        PWRSMEN_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbsmenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbsmenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBSMENR1_SPEC;
impl crate::RegisterSpec for APBSMENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr1::R`](R) reader structure"]
impl crate::Readable for APBSMENR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr1::W`](W) writer structure"]
impl crate::Writable for APBSMENR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBSMENR1 to value 0xffff_ffb7"]
impl crate::Resettable for APBSMENR1_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffb7;
}
