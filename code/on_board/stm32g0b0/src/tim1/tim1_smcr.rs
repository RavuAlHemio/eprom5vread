#[doc = "Register `TIM1_SMCR` reader"]
pub type R = crate::R<TIM1_SMCR_SPEC>;
#[doc = "Register `TIM1_SMCR` writer"]
pub type W = crate::W<TIM1_SMCR_SPEC>;
#[doc = "Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_R = crate::FieldReader<SMS1_A>;
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS1_A {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B_0X1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    B_0X2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    B_0X3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    B_0X4 = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    B_0X5 = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    B_0X6 = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    B_0X7 = 7,
}
impl From<SMS1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMS1_A {
    type Ux = u8;
}
impl SMS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMS1_A {
        match self.bits {
            0 => SMS1_A::B_0X0,
            1 => SMS1_A::B_0X1,
            2 => SMS1_A::B_0X2,
            3 => SMS1_A::B_0X3,
            4 => SMS1_A::B_0X4,
            5 => SMS1_A::B_0X5,
            6 => SMS1_A::B_0X6,
            7 => SMS1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS1_A::B_0X0
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMS1_A::B_0X1
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SMS1_A::B_0X2
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SMS1_A::B_0X3
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SMS1_A::B_0X4
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SMS1_A::B_0X5
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == SMS1_A::B_0X6
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == SMS1_A::B_0X7
    }
}
#[doc = "Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, SMS1_A>;
impl<'a, REG> SMS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X0)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI2FP2 edge depending on TI1FP1 level."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X4)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X5)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X6)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(SMS1_A::B_0X7)
    }
}
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OCCS_R = crate::BitReader<OCCS_A>;
#[doc = "OCREF clear selection This bit is used to select the OCREF clear source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OCCS_A {
    #[doc = "0: OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    B_0X0 = 0,
    #[doc = "1: OCREF_CLR_INT is connected to ETRF"]
    B_0X1 = 1,
}
impl From<OCCS_A> for bool {
    #[inline(always)]
    fn from(variant: OCCS_A) -> Self {
        variant as u8 != 0
    }
}
impl OCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCCS_A {
        match self.bits {
            false => OCCS_A::B_0X0,
            true => OCCS_A::B_0X1,
        }
    }
    #[doc = "OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OCCS_A::B_0X0
    }
    #[doc = "OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OCCS_A::B_0X1
    }
}
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source."]
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG, OCCS_A>;
impl<'a, REG> OCCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCREF_CLR_INT is connected to COMP1 or COMP2 output depending on TIM1_OR1.OCREF_CLR"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS_A::B_0X0)
    }
    #[doc = "OCREF_CLR_INT is connected to ETRF"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OCCS_A::B_0X1)
    }
}
#[doc = "Field `TS1` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_R = crate::FieldReader<TS1_A>;
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS1_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B_0X0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B_0X1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B_0X2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B_0X3 = 3,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    B_0X4 = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    B_0X5 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    B_0X6 = 6,
    #[doc = "7: External Trigger input (ETRF)"]
    B_0X7 = 7,
}
impl From<TS1_A> for u8 {
    #[inline(always)]
    fn from(variant: TS1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS1_A {
    type Ux = u8;
}
impl TS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1_A {
        match self.bits {
            0 => TS1_A::B_0X0,
            1 => TS1_A::B_0X1,
            2 => TS1_A::B_0X2,
            3 => TS1_A::B_0X3,
            4 => TS1_A::B_0X4,
            5 => TS1_A::B_0X5,
            6 => TS1_A::B_0X6,
            7 => TS1_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TS1_A::B_0X0
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TS1_A::B_0X1
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TS1_A::B_0X2
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TS1_A::B_0X3
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TS1_A::B_0X4
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TS1_A::B_0X5
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == TS1_A::B_0X6
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == TS1_A::B_0X7
    }
}
#[doc = "Field `TS1` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TS1_A>;
impl<'a, REG> TS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X3)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X4)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X5)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X6)
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(TS1_A::B_0X7)
    }
}
#[doc = "Field `MSM` reader - Master/slave mode"]
pub type MSM_R = crate::BitReader<MSM_A>;
#[doc = "Master/slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    B_0X0 = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    B_0X1 = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::B_0X0,
            true => MSM_A::B_0X1,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSM_A::B_0X0
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSM_A::B_0X1
    }
}
#[doc = "Field `MSM` writer - Master/slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG, MSM_A>;
impl<'a, REG> MSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::B_0X0)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSM_A::B_0X1)
    }
}
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_R = crate::FieldReader<ETF_A>;
#[doc = "External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    B_0X0 = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    B_0X1 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    B_0X2 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    B_0X3 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    B_0X4 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    B_0X5 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    B_0X6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    B_0X7 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    B_0X8 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    B_0X9 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    B_0X_A = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    B_0X_B = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    B_0X_C = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    B_0X_D = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    B_0X_E = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    B_0X_F = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETF_A {
    type Ux = u8;
}
impl ETF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::B_0X0,
            1 => ETF_A::B_0X1,
            2 => ETF_A::B_0X2,
            3 => ETF_A::B_0X3,
            4 => ETF_A::B_0X4,
            5 => ETF_A::B_0X5,
            6 => ETF_A::B_0X6,
            7 => ETF_A::B_0X7,
            8 => ETF_A::B_0X8,
            9 => ETF_A::B_0X9,
            10 => ETF_A::B_0X_A,
            11 => ETF_A::B_0X_B,
            12 => ETF_A::B_0X_C,
            13 => ETF_A::B_0X_D,
            14 => ETF_A::B_0X_E,
            15 => ETF_A::B_0X_F,
            _ => unreachable!(),
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETF_A::B_0X0
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETF_A::B_0X1
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETF_A::B_0X2
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETF_A::B_0X3
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == ETF_A::B_0X4
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ETF_A::B_0X5
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == ETF_A::B_0X6
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == ETF_A::B_0X7
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == ETF_A::B_0X8
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == ETF_A::B_0X9
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == ETF_A::B_0X_A
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == ETF_A::B_0X_B
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == ETF_A::B_0X_C
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == ETF_A::B_0X_D
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == ETF_A::B_0X_E
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == ETF_A::B_0X_F
    }
}
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, ETF_A>;
impl<'a, REG> ETF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_A)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_B)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_C)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_D)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_E)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(ETF_A::B_0X_F)
    }
}
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type ETPS_R = crate::FieldReader<ETPS_A>;
#[doc = "External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS_A {
    #[doc = "0: Prescaler OFF"]
    B_0X0 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    B_0X1 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    B_0X2 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    B_0X3 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETPS_A {
    type Ux = u8;
}
impl ETPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::B_0X0,
            1 => ETPS_A::B_0X1,
            2 => ETPS_A::B_0X2,
            3 => ETPS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETPS_A::B_0X0
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETPS_A::B_0X1
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETPS_A::B_0X2
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETPS_A::B_0X3
    }
}
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type ETPS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ETPS_A>;
impl<'a, REG> ETPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0X0)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0X1)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0X2)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ETPS_A::B_0X3)
    }
}
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type ECE_R = crate::BitReader<ECE_A>;
#[doc = "External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE_A {
    #[doc = "0: External clock mode 2 disabled"]
    B_0X0 = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    B_0X1 = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::B_0X0,
            true => ECE_A::B_0X1,
        }
    }
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECE_A::B_0X0
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECE_A::B_0X1
    }
}
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG, ECE_A>;
impl<'a, REG> ECE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECE_A::B_0X0)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECE_A::B_0X1)
    }
}
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type ETP_R = crate::BitReader<ETP_A>;
#[doc = "External trigger polarity This bit selects whether ETR or ETR is used for trigger operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP_A {
    #[doc = "0: ETR is non-inverted, active at high level or rising edge."]
    B_0X0 = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge."]
    B_0X1 = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::B_0X0,
            true => ETP_A::B_0X1,
        }
    }
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETP_A::B_0X0
    }
    #[doc = "ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETP_A::B_0X1
    }
}
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG, ETP_A>;
impl<'a, REG> ETP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ETR is non-inverted, active at high level or rising edge."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::B_0X0)
    }
    #[doc = "ETR is inverted, active at low level or falling edge."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETP_A::B_0X1)
    }
}
#[doc = "Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_R = crate::BitReader<SMS2_A>;
#[doc = "Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMS2_A {
    #[doc = "0: Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    B_0X0 = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    B_0X1 = 1,
}
impl From<SMS2_A> for bool {
    #[inline(always)]
    fn from(variant: SMS2_A) -> Self {
        variant as u8 != 0
    }
}
impl SMS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMS2_A {
        match self.bits {
            false => SMS2_A::B_0X0,
            true => SMS2_A::B_0X1,
        }
    }
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMS2_A::B_0X0
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMS2_A::B_0X1
    }
}
#[doc = "Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_W<'a, REG> = crate::BitWriter<'a, REG, SMS2_A>;
impl<'a, REG> SMS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled - if CEN = '1' then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMS2_A::B_0X0)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI1FP1 edge depending on TI2FP2 level."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMS2_A::B_0X1)
    }
}
#[doc = "Field `TS2` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_R = crate::FieldReader<TS2_A>;
#[doc = "Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS2_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    B_0X0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    B_0X1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    B_0X2 = 2,
    #[doc = "3: Internal Trigger 3 (ITR3)"]
    B_0X3 = 3,
}
impl From<TS2_A> for u8 {
    #[inline(always)]
    fn from(variant: TS2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS2_A {
    type Ux = u8;
}
impl TS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS2_A {
        match self.bits {
            0 => TS2_A::B_0X0,
            1 => TS2_A::B_0X1,
            2 => TS2_A::B_0X2,
            3 => TS2_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TS2_A::B_0X0
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TS2_A::B_0X1
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TS2_A::B_0X2
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TS2_A::B_0X3
    }
}
#[doc = "Field `TS2` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TS2_A>;
impl<'a, REG> TS2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TS2_A::B_0X0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TS2_A::B_0X1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TS2_A::B_0X2)
    }
    #[doc = "Internal Trigger 3 (ITR3)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TS2_A::B_0X3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> SMS1_W<TIM1_SMCR_SPEC> {
        SMS1_W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source."]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<TIM1_SMCR_SPEC> {
        OCCS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<TIM1_SMCR_SPEC> {
        TS1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<TIM1_SMCR_SPEC> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<TIM1_SMCR_SPEC> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of fCK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<TIM1_SMCR_SPEC> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<TIM1_SMCR_SPEC> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<TIM1_SMCR_SPEC> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> SMS2_W<TIM1_SMCR_SPEC> {
        SMS2_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<TIM1_SMCR_SPEC> {
        TS2_W::new(self, 20)
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
#[doc = "slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim1_smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim1_smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM1_SMCR_SPEC;
impl crate::RegisterSpec for TIM1_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim1_smcr::R`](R) reader structure"]
impl crate::Readable for TIM1_SMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim1_smcr::W`](W) writer structure"]
impl crate::Writable for TIM1_SMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM1_SMCR to value 0"]
impl crate::Resettable for TIM1_SMCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
