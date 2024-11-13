use core::time::Duration;
use core::{
    num::{NonZeroU32,NonZeroU64, NonZeroU128},
    ops::{Div,Add},
    arch::asm,
    ptr::addr_of_mut
};

use crate::println;

const NANOSEC_PER_SEC: NonZeroU64 = unsafe {NonZeroU64::new_unchecked(1_000_000_000)};

#[no_mangle]
static mut ARCH_TIMER_COUNTER_FREQ:NonZeroU32 = NonZeroU32::MIN;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct GenericTimerCounterValue(u64);

impl TryFrom<Duration> for GenericTimerCounterValue {
    type Error = &'static str;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        if duration < resolution() {
            return Ok(GenericTimerCounterValue(0));
        }

        if duration > max_duration() {
            return Err("Conversion error. Duration too big");
        }

        let frequency: u128 = u32::from(arch_timer_counter_frequency()) as u128;
        let duration: u128 = duration.as_nanos();

        // This is safe, because frequency can never be greater than u32::MAX, and
        // (Duration::MAX.as_nanos() * u32::MAX) < u128::MAX.
        let counter_value =
            match duration.checked_mul(frequency) {
                Some(dur) => dur.div(NonZeroU128::from(NANOSEC_PER_SEC)),
                None => {
                    panic!("overflow occurred");
                }
            };

        // Since we checked above that we are <= max_duration(), just cast to u64.
        Ok(GenericTimerCounterValue(counter_value as u64))
    }
}

impl GenericTimerCounterValue {
    pub const MAX: Self = GenericTimerCounterValue(u64::MAX);
}


fn arch_timer_counter_frequency() -> NonZeroU32 {
    // Read volatile is needed here to prevent the compiler from optimizing
    // ARCH_TIMER_COUNTER_FREQUENCY away.
    unsafe { 
        core::ptr::read_volatile(addr_of_mut!(ARCH_TIMER_COUNTER_FREQ))
    }
}

impl From<GenericTimerCounterValue> for Duration {
    fn from(counter_value: GenericTimerCounterValue) -> Self {
        if counter_value.0 == 0 {
            return Duration::ZERO;
        }

        let frequency: NonZeroU64 = arch_timer_counter_frequency().into();

        // Div<NonZeroU64> implementation for u64 cannot panic.
        let secs = counter_value.0.div(frequency);

        // This is safe, because frequency can never be greater than u32::MAX, which means the
        // largest theoretical value for sub_second_counter_value is (u32::MAX - 1). Therefore,
        // (sub_second_counter_value * NANOSEC_PER_SEC) cannot overflow an u64.
        //
        // The subsequent division ensures the result fits into u32, since the max result is smaller
        // than NANOSEC_PER_SEC. Therefore, just cast it to u32 using `as`.
        let sub_second_counter_value  = counter_value.0 % frequency;
        let nanos = sub_second_counter_value.checked_mul(u64::from(NANOSEC_PER_SEC))
                        .and_then(|value| value.checked_div(frequency.get()))
                        .map(|result| result as u32)
                        .unwrap_or(0);

        Duration::new(secs, nanos)
    }
}

fn max_duration() -> Duration {
    Duration::from(GenericTimerCounterValue::MAX)
}

pub fn resolution() -> Duration {
    Duration::from(GenericTimerCounterValue(1))
}

impl Add<GenericTimerCounterValue> for u64 {
    type Output = u64;

    fn add(self, other: GenericTimerCounterValue) -> u64 {
        self.wrapping_add(other.0)
    }
}


pub fn spin_for(duration:Duration) {
    let mut cnt_start:u64;
    let mut cnt_cur:u64;

    unsafe {
        asm!("mrs {0}, CNTPCT_EL0", out(reg) cnt_start);
    };

    let counter_value_delta: GenericTimerCounterValue = match duration.try_into() {
        Err(msg) => {
            println!("spin_for: {}. Skipping", msg);
            return;
        }
        Ok(val) => val,
    };

    let cnt_end:u64 = cnt_start + counter_value_delta;

    while {
        unsafe {
            asm!("mrs {0}, CNTPCT_EL0", out(reg) cnt_cur);
        }
        cnt_cur < cnt_end
     } {}
}