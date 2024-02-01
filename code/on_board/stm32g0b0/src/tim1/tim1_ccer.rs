#[doc = "Register `TIM1_CCER` reader"]
pub type R = crate::R<TIM1_CCER_SPEC>;
#[doc = "Register `TIM1_CCER` writer"]
pub type W = crate::W<TIM1_CCER_SPEC>;
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1E_R = crate::BitReader<CC1E_A>;
#[doc = "Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1E_A {
    #[doc = "0: Capture mode disabled / OC1 is not active (see below)"]
    B_0X0 = 0,
    #[doc = "1: Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    B_0X1 = 1,
}
impl From<CC1E_A> for bool {
    #[inline(always)]
    fn from(variant: CC1E_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1E_A {
        match self.bits {
            false => CC1E_A::B_0X0,
            true => CC1E_A::B_0X1,
        }
    }
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1E_A::B_0X0
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1E_A::B_0X1
    }
}
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG, CC1E_A>;
impl<'a, REG> CC1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture mode disabled / OC1 is not active (see below)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0X0)
    }
    #[doc = "Capture mode enabled / OC1 signal is output on the corresponding output pin"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1E_A::B_0X1)
    }
}
#[doc = "Field `CC1P` reader - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1P_R = crate::BitReader<CC1P_A>;
#[doc = "Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1P_A {
    #[doc = "0: OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    B_0X0 = 0,
    #[doc = "1: OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    B_0X1 = 1,
}
impl From<CC1P_A> for bool {
    #[inline(always)]
    fn from(variant: CC1P_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1P_A {
        match self.bits {
            false => CC1P_A::B_0X0,
            true => CC1P_A::B_0X1,
        }
    }
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1P_A::B_0X0
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1P_A::B_0X1
    }
}
#[doc = "Field `CC1P` writer - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG, CC1P_A>;
impl<'a, REG> CC1P_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1 active high (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0X0)
    }
    #[doc = "OC1 active low (output mode) / Edge sensitivity selection (input mode, see below)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1P_A::B_0X1)
    }
}
#[doc = "Field `CC1NE` reader - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NE_R = crate::BitReader<CC1NE_A>;
#[doc = "Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NE_A {
    #[doc = "0: Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B_0X0 = 0,
    #[doc = "1: On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    B_0X1 = 1,
}
impl From<CC1NE_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NE_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1NE_A {
        match self.bits {
            false => CC1NE_A::B_0X0,
            true => CC1NE_A::B_0X1,
        }
    }
    #[doc = "Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NE_A::B_0X0
    }
    #[doc = "On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NE_A::B_0X1
    }
}
#[doc = "Field `CC1NE` writer - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG, CC1NE_A>;
impl<'a, REG> CC1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off - OC1N is not active. OC1N level is then function of MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE_A::B_0X0)
    }
    #[doc = "On - OC1N signal is output on the corresponding output pin depending on MOE, OSSI, OSSR, OIS1, OIS1N and CC1E bits."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NE_A::B_0X1)
    }
}
#[doc = "Field `CC1NP` reader - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NP_R = crate::BitReader<CC1NP_A>;
#[doc = "Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1NP_A {
    #[doc = "0: OC1N active high."]
    B_0X0 = 0,
    #[doc = "1: OC1N active low."]
    B_0X1 = 1,
}
impl From<CC1NP_A> for bool {
    #[inline(always)]
    fn from(variant: CC1NP_A) -> Self {
        variant as u8 != 0
    }
}
impl CC1NP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1NP_A {
        match self.bits {
            false => CC1NP_A::B_0X0,
            true => CC1NP_A::B_0X1,
        }
    }
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1NP_A::B_0X0
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1NP_A::B_0X1
    }
}
#[doc = "Field `CC1NP` writer - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG, CC1NP_A>;
impl<'a, REG> CC1NP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N active high."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP_A::B_0X0)
    }
    #[doc = "OC1N active low."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1NP_A::B_0X1)
    }
}
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable Refer to CC1E description"]
pub type CC2E_R = crate::BitReader;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable Refer to CC1E description"]
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output polarity Refer to CC1P description"]
pub type CC2P_R = crate::BitReader;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output polarity Refer to CC1P description"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
pub type CC2NE_R = crate::BitReader;
#[doc = "Field `CC2NE` writer - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
pub type CC2NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
pub type CC2NP_R = crate::BitReader;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable Refer to CC1E description"]
pub type CC3E_R = crate::BitReader;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable Refer to CC1E description"]
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output polarity Refer to CC1P description"]
pub type CC3P_R = crate::BitReader;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output polarity Refer to CC1P description"]
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
pub type CC3NE_R = crate::BitReader;
#[doc = "Field `CC3NE` writer - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
pub type CC3NP_R = crate::BitReader;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable Refer to CC1E description"]
pub type CC4E_R = crate::BitReader;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable Refer to CC1E description"]
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Capture/Compare 4 output polarity Refer to CC1P description"]
pub type CC4P_R = crate::BitReader;
#[doc = "Field `CC4P` writer - Capture/Compare 4 output polarity Refer to CC1P description"]
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
pub type CC4NP_R = crate::BitReader;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5E` reader - Capture/Compare 5 output enable Refer to CC1E description"]
pub type CC5E_R = crate::BitReader;
#[doc = "Field `CC5E` writer - Capture/Compare 5 output enable Refer to CC1E description"]
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5P` reader - Capture/Compare 5 output polarity Refer to CC1P description"]
pub type CC5P_R = crate::BitReader;
#[doc = "Field `CC5P` writer - Capture/Compare 5 output polarity Refer to CC1P description"]
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6E` reader - Capture/Compare 6 output enable Refer to CC1E description"]
pub type CC6E_R = crate::BitReader;
#[doc = "Field `CC6E` writer - Capture/Compare 6 output enable Refer to CC1E description"]
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC6P` reader - Capture/Compare 6 output polarity Refer to CC1P description"]
pub type CC6P_R = crate::BitReader;
#[doc = "Field `CC6P` writer - Capture/Compare 6 output polarity Refer to CC1P description"]
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/Compare 1 output enable When CC1 channel is configured as output, the OC1 level depends on MOE, OSSI, OSSR, OIS1, OIS1N and CC1NE bits, regardless of the CC1E bits state. Refer to for details. Note: On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1E active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<TIM1_CCER_SPEC> {
        CC1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output polarity When CC1 channel is configured as input, both CC1NP/CC1P bits select the active polarity of TI1FP1 and TI2FP1 for trigger or capture operations. CC1NP=0, CC1P=0: non-inverted/rising edge. The circuit is sensitive to TIxFP1 rising edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is not inverted (trigger operation in gated mode or encoder mode). CC1NP=0, CC1P=1: inverted/falling edge. The circuit is sensitive to TIxFP1 falling edge (capture or trigger operations in reset, external clock or trigger mode), TIxFP1 is inverted (trigger operation in gated mode or encoder mode). CC1NP=1, CC1P=1: non-inverted/both edges/ The circuit is sensitive to both TIxFP1 rising and falling edges (capture or trigger operations in reset, external clock or trigger mode), TIxFP1is not inverted (trigger operation in gated mode). This configuration must not be used in encoder mode. CC1NP=1, CC1P=0: The configuration is reserved, it must not be used. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1P active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIM1_CCER_SPEC> {
        CC1P_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 1 complementary output enable On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NE active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<TIM1_CCER_SPEC> {
        CC1NE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 1 complementary output polarity CC1 channel configured as output: CC1 channel configured as input: This bit is used in conjunction with CC1P to define the polarity of TI1FP1 and TI2FP1. Refer to CC1P description. Note: This bit is not writable as soon as LOCK level 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register) and CC1S='00' (channel configured as output). On channels having a complementary output, this bit is preloaded. If the CCPC bit is set in the TIMx_CR2 register then the CC1NP active bit takes the new value from the preloaded bit only when a Commutation event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIM1_CCER_SPEC> {
        CC1NP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<TIM1_CCER_SPEC> {
        CC2E_W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<TIM1_CCER_SPEC> {
        CC2P_W::new(self, 5)
    }
    #[doc = "Bit 6 - Capture/Compare 2 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ne(&mut self) -> CC2NE_W<TIM1_CCER_SPEC> {
        CC2NE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture/Compare 2 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<TIM1_CCER_SPEC> {
        CC2NP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<TIM1_CCER_SPEC> {
        CC3E_W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<TIM1_CCER_SPEC> {
        CC3P_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 3 complementary output enable Refer to CC1NE description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> CC3NE_W<TIM1_CCER_SPEC> {
        CC3NE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<TIM1_CCER_SPEC> {
        CC3NP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<TIM1_CCER_SPEC> {
        CC4E_W::new(self, 12)
    }
    #[doc = "Bit 13 - Capture/Compare 4 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<TIM1_CCER_SPEC> {
        CC4P_W::new(self, 13)
    }
    #[doc = "Bit 15 - Capture/Compare 4 complementary output polarity Refer to CC1NP description"]
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> CC4NP_W<TIM1_CCER_SPEC> {
        CC4NP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Capture/Compare 5 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc5e(&mut self) -> CC5E_W<TIM1_CCER_SPEC> {
        CC5E_W::new(self, 16)
    }
    #[doc = "Bit 17 - Capture/Compare 5 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc5p(&mut self) -> CC5P_W<TIM1_CCER_SPEC> {
        CC5P_W::new(self, 17)
    }
    #[doc = "Bit 20 - Capture/Compare 6 output enable Refer to CC1E description"]
    #[inline(always)]
    #[must_use]
    pub fn cc6e(&mut self) -> CC6E_W<TIM1_CCER_SPEC> {
        CC6E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Capture/Compare 6 output polarity Refer to CC1P description"]
    #[inline(always)]
    #[must_use]
    pub fn cc6p(&mut self) -> CC6P_W<TIM1_CCER_SPEC> {
        CC6P_W::new(self, 21)
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
#[doc = "capture/compare enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_ccer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_ccer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_CCER_SPEC;
impl crate::RegisterSpec for TIM1_CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_ccer::R`](R) reader structure"]
impl crate::Readable for TIM1_CCER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_ccer::W`](W) writer structure"]
impl crate::Writable for TIM1_CCER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CCER to value 0"]
impl crate::Resettable for TIM1_CCER_SPEC {
    const RESET_VALUE: u32 = 0;
}
