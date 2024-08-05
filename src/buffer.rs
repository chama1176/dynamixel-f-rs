/// BUFFER SIZE should be lower than u32 max -1 and 2...
pub struct RingBuffer<const BUFFER_SIZE: usize> {
    rp_: usize, // read pointer
    wp_: usize, // write pointe
    buffer_: [u8; BUFFER_SIZE],
}

impl<const BUFFER_SIZE: usize> RingBuffer<BUFFER_SIZE> {
    pub fn new() -> Self {
        Self {
            rp_: 0,
            wp_: 0,
            buffer_: [0; BUFFER_SIZE],
        }
    }
    pub fn enqueue(&mut self, data: u8) -> Result<(), ()> {
        if ((self.wp_ - self.rp_) & (BUFFER_SIZE - 1)) == (BUFFER_SIZE - 1) {
            return Err(());
        }
        self.buffer_[self.wp_] = data;
        self.inc_wp();
        Ok(())
    }
    pub fn dequeue(&mut self) -> Option<u8> {
        if self.rp_ == self.wp_ {
            return None;
        }
        let data = self.buffer_[self.rp_];
        self.inc_rp();
        Some(data)
    }
    pub fn is_empty(&self) -> bool {
        if self.rp_ == self.wp_ {
            true
        } else {
            false
        }
    }
    fn inc_wp(&mut self) {
        self.wp_ = (self.wp_ + 1) & (BUFFER_SIZE - 1);
    }
    fn inc_rp(&mut self) {
        self.rp_ = (self.rp_ + 1) & (BUFFER_SIZE - 1);
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::{self, RingBuffer};

    #[test]
    fn enqueue_and_dequeue() {
        let mut r = RingBuffer::<128>::new();
        // Should be empty at first.
        assert_eq!(r.dequeue(), None);
        let _ = r.enqueue(1);
        assert_eq!(r.dequeue(), Some(1));
    }

    #[test]
    fn enqueue() {
        let mut r = RingBuffer::<128>::new();
        assert_eq!(1 + 1, 2);
    }
}
