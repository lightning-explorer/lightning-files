use std::time::Duration;

#[derive(Clone)]
pub enum ThrottleAmount {
    None,
    Low,
    Medium,
    High,
}
#[derive(Clone)]
pub struct CrawlerThrottle {
    duration: Duration,
    amount: ThrottleAmount,
}

impl CrawlerThrottle {
    /// Initialize a new throttler with the throttle set to `None`
    pub fn new() -> Self {
        let amount = ThrottleAmount::None;
        let duration = Self::duration_for_throttle(&amount);
        Self { duration, amount }
    }
    pub fn set<I>(&mut self, amount: I)
    where
        I: Into<ThrottleAmount>,
    {
        self.amount = amount.into();
    }
    pub fn upgrade(&mut self) {
        match self.amount {
            ThrottleAmount::None => self.amount = ThrottleAmount::Low,
            ThrottleAmount::Low => self.amount = ThrottleAmount::Medium,
            ThrottleAmount::Medium => self.amount = ThrottleAmount::High,
            ThrottleAmount::High => {}
        }
    }
    pub fn downgrade(&mut self) {
        match self.amount {
            ThrottleAmount::None => {}
            ThrottleAmount::Low => self.amount = ThrottleAmount::None,
            ThrottleAmount::Medium => self.amount = ThrottleAmount::Low,
            ThrottleAmount::High => self.amount = ThrottleAmount::Medium,
        }
    }
    /// Sets the throttle to `None`
    pub fn reset(&mut self) {
        self.amount = ThrottleAmount::None;
    }
    /// If the throttle amount is set to `None`, then no rest happens
    pub async fn rest_short(&self) {
        if let ThrottleAmount::None = self.amount {
            return;
        }
        tokio::time::sleep(self.duration).await
    }
    fn duration_for_throttle(amount: &ThrottleAmount) -> Duration {
        match amount {
            ThrottleAmount::None => Duration::from_millis(0),
            ThrottleAmount::Low => Duration::from_millis(50),
            ThrottleAmount::Medium => Duration::from_millis(100),
            ThrottleAmount::High => Duration::from_millis(200),
        }
    }
}

impl From<CrawlerThrottle> for ThrottleAmount {
    fn from(value: CrawlerThrottle) -> Self {
        value.amount
    }
}
