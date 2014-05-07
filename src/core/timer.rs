use unit::MS;

pub struct Timer {
    is_active: bool,
    elapsed: MS,
    expiration: MS,
}

impl Timer {
    pub fn new(expiration: MS) -> Timer {
        Timer {
            is_active: false,
            elapsed: MS(0),
            expiration: expiration,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.elapsed >= self.expiration
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn elapsed(&self) -> MS {
        self.elapsed
    }

    pub fn remain(&self) -> MS {
        self.expiration - self.elapsed
    }

    pub fn reset(&mut self) {
        self.is_active = true;
        self.elapsed = MS(0);
    }

    pub fn update(&mut self, dt: MS) {
        if self.is_active() {
            self.elapsed = self.elapsed + dt;
            if self.is_expired() {
                self.is_active = false;
                self.elapsed = self.expiration;
            }
        }
    }
}
