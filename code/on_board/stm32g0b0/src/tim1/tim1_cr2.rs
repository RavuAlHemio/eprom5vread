#[doc = "Register `TIM1_CR2` reader"]
pub type R = crate::R<TIM1_CR2_SPEC>;
#[doc = "Register `TIM1_CR2` writer"]
pub type W = crate::W<TIM1_CR2_SPEC>;
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader<CCPC_A>;
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC_A {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    B_0X0 = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    B_0X1 = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::B_0X0,
            true => CCPC_A::B_0X1,
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCPC_A::B_0X0
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCPC_A::B_0X1
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG, CCPC_A>;
impl<'a, REG> CCPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC_A::B_0X0)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded, after having been written, they are updated only when a commutation event (COM) occurs (COMG bit set or rising edge detected on TRGI, depending on the CCUS bit)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC_A::B_0X1)
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader<CCUS_A>;
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS_A {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    B_0X0 = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    B_0X1 = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::B_0X0,
            true => CCUS_A::B_0X1,
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCUS_A::B_0X0
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCUS_A::B_0X1
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS_A>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS_A::B_0X0)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS_A::B_0X1)
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS_A>;
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    B_0X0 = 0,
    #[doc = "1: CCx DMA requests sent when update event occurs"]
    B_0X1 = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::B_0X0,
            true => CCDS_A::B_0X1,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCDS_A::B_0X0
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCDS_A::B_0X1
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS_A>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS_A::B_0X0)
    }
    #[doc = "CCx DMA requests sent when update event occurs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS_A::B_0X1)
    }
}
#[doc = "Field `MMS` reader - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<MMS_A>;
#[doc = "Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    B_0X0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0X1 = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    B_0X2 = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    B_0X3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO)"]
    B_0X4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO)"]
    B_0X5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO)"]
    B_0X6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO)"]
    B_0X7 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS_A {
    type Ux = u8;
}
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::B_0X0,
            1 => MMS_A::B_0X1,
            2 => MMS_A::B_0X2,
            3 => MMS_A::B_0X3,
            4 => MMS_A::B_0X4,
            5 => MMS_A::B_0X5,
            6 => MMS_A::B_0X6,
            7 => MMS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS_A::B_0X0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MMS_A::B_0X1
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MMS_A::B_0X2
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MMS_A::B_0X3
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MMS_A::B_0X4
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MMS_A::B_0X5
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MMS_A::B_0X6
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MMS_A::B_0X7
    }
}
#[doc = "Field `MMS` writer - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, MMS_A>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enable. The Counter Enable signal is generated by a logic AND between CEN control bit and the trigger input when configured in gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X1)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X2)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred. (TRGO)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS_A::B_0X7)
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type TI1S_R = crate::BitReader<TI1S_A>;
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S_A {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    B_0X0 = 0,
    #[doc = "1: The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    B_0X1 = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::B_0X0,
            true => TI1S_A::B_0X1,
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1S_A::B_0X0
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1S_A::B_0X1
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S_A>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S_A::B_0X0)
    }
    #[doc = "The TIMx_CH1, CH2 and CH3 pins are connected to the TI1 input (XOR combination)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S_A::B_0X1)
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_R = crate::BitReader<OIS1_A>;
#[doc = "Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1_A {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    B_0X0 = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    B_0X1 = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl OIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::B_0X0,
            true => OIS1_A::B_0X1,
        }
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIS1_A::B_0X0
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIS1_A::B_0X1
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG, OIS1_A>;
impl<'a, REG> OIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1_A::B_0X0)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1_A::B_0X1)
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_R = crate::BitReader<OIS1N_A>;
#[doc = "Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N_A {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    B_0X0 = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    B_0X1 = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
impl OIS1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::B_0X0,
            true => OIS1N_A::B_0X1,
        }
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OIS1N_A::B_0X0
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OIS1N_A::B_0X1
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N_A>;
impl<'a, REG> OIS1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N_A::B_0X0)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N_A::B_0X1)
    }
}
#[doc = "Field `OIS2` reader - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type OIS2_R = crate::BitReader;
#[doc = "Field `OIS2` writer - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
pub type OIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type OIS2N_R = crate::BitReader;
#[doc = "Field `OIS2N` writer - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
pub type OIS2N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type OIS3_R = crate::BitReader;
#[doc = "Field `OIS3` writer - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
pub type OIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type OIS3N_R = crate::BitReader;
#[doc = "Field `OIS3N` writer - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
pub type OIS3N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type OIS4_R = crate::BitReader;
#[doc = "Field `OIS4` writer - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
pub type OIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS5` reader - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type OIS5_R = crate::BitReader;
#[doc = "Field `OIS5` writer - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
pub type OIS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS6` reader - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type OIS6_R = crate::BitReader;
#[doc = "Field `OIS6` writer - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
pub type OIS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::FieldReader<MMS2_A>;
#[doc = "Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS2_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    B_0X0 = 0,
    #[doc = "1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    B_0X1 = 1,
    #[doc = "2: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    B_0X2 = 2,
    #[doc = "3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    B_0X3 = 3,
    #[doc = "4: Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    B_0X4 = 4,
    #[doc = "5: Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    B_0X5 = 5,
    #[doc = "6: Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    B_0X6 = 6,
    #[doc = "7: Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    B_0X7 = 7,
    #[doc = "8: Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    B_0X8 = 8,
    #[doc = "9: Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    B_0X9 = 9,
    #[doc = "10: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    B_0X_A = 10,
    #[doc = "11: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    B_0X_B = 11,
    #[doc = "12: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B_0X_C = 12,
    #[doc = "13: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B_0X_D = 13,
    #[doc = "14: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    B_0X_E = 14,
    #[doc = "15: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    B_0X_F = 15,
}
impl From<MMS2_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS2_A {
    type Ux = u8;
}
impl MMS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMS2_A {
        match self.bits {
            0 => MMS2_A::B_0X0,
            1 => MMS2_A::B_0X1,
            2 => MMS2_A::B_0X2,
            3 => MMS2_A::B_0X3,
            4 => MMS2_A::B_0X4,
            5 => MMS2_A::B_0X5,
            6 => MMS2_A::B_0X6,
            7 => MMS2_A::B_0X7,
            8 => MMS2_A::B_0X8,
            9 => MMS2_A::B_0X9,
            10 => MMS2_A::B_0X_A,
            11 => MMS2_A::B_0X_B,
            12 => MMS2_A::B_0X_C,
            13 => MMS2_A::B_0X_D,
            14 => MMS2_A::B_0X_E,
            15 => MMS2_A::B_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MMS2_A::B_0X0
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MMS2_A::B_0X1
    }
    #[doc = "Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MMS2_A::B_0X2
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MMS2_A::B_0X3
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MMS2_A::B_0X4
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MMS2_A::B_0X5
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MMS2_A::B_0X6
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MMS2_A::B_0X7
    }
    #[doc = "Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MMS2_A::B_0X8
    }
    #[doc = "Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MMS2_A::B_0X9
    }
    #[doc = "Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MMS2_A::B_0X_A
    }
    #[doc = "Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == MMS2_A::B_0X_B
    }
    #[doc = "Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == MMS2_A::B_0X_C
    }
    #[doc = "Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == MMS2_A::B_0X_D
    }
    #[doc = "Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == MMS2_A::B_0X_E
    }
    #[doc = "Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == MMS2_A::B_0X_F
    }
}
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, MMS2_A>;
impl<'a, REG> MMS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X0)
    }
    #[doc = "Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X1)
    }
    #[doc = "Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X2)
    }
    #[doc = "Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X3)
    }
    #[doc = "Compare - OC1REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X4)
    }
    #[doc = "Compare - OC2REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X5)
    }
    #[doc = "Compare - OC3REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X6)
    }
    #[doc = "Compare - OC4REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X7)
    }
    #[doc = "Compare - OC5REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X8)
    }
    #[doc = "Compare - OC6REFC signal is used as trigger output (TRGO2)"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X9)
    }
    #[doc = "Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_A)
    }
    #[doc = "Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_B)
    }
    #[doc = "Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_C)
    }
    #[doc = "Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_D)
    }
    #[doc = "Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_E)
    }
    #[doc = "Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(MMS2_A::B_0X_F)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<TIM1_CR2_SPEC> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<TIM1_CR2_SPEC> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<TIM1_CR2_SPEC> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Master mode selection These bits allow selected information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<TIM1_CR2_SPEC> {
        MMS_W::new(self, 4)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<TIM1_CR2_SPEC> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - Output Idle state 1 (OC1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<TIM1_CR2_SPEC> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1 (OC1N output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<TIM1_CR2_SPEC> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output Idle state 2 (OC2 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<TIM1_CR2_SPEC> {
        OIS2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output Idle state 2 (OC2N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<TIM1_CR2_SPEC> {
        OIS2N_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output Idle state 3 (OC3 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<TIM1_CR2_SPEC> {
        OIS3_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output Idle state 3 (OC3N output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<TIM1_CR2_SPEC> {
        OIS3N_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output Idle state 4 (OC4 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<TIM1_CR2_SPEC> {
        OIS4_W::new(self, 14)
    }
    #[doc = "Bit 16 - Output Idle state 5 (OC5 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<TIM1_CR2_SPEC> {
        OIS5_W::new(self, 16)
    }
    #[doc = "Bit 18 - Output Idle state 6 (OC6 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<TIM1_CR2_SPEC> {
        OIS6_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (TRGO2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<TIM1_CR2_SPEC> {
        MMS2_W::new(self, 20)
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
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_CR2_SPEC;
impl crate::RegisterSpec for TIM1_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_cr2::R`](R) reader structure"]
impl crate::Readable for TIM1_CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_cr2::W`](W) writer structure"]
impl crate::Writable for TIM1_CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_CR2 to value 0"]
impl crate::Resettable for TIM1_CR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
