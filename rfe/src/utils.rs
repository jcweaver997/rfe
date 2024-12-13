
pub struct ManualAuto<T: Clone + PartialEq> {
    value_auto: T,
    value_manual: T,
    is_manual: bool,
    has_changed: bool,
}

impl<T: Clone + PartialEq> ManualAuto<T> {
    pub fn new(value: T, is_manual: bool) -> Self {
        Self {
            value_auto: value.clone(),
            value_manual: value,
            is_manual,
            has_changed: false,
        }
    }

    pub fn auto_set(&mut self, value: T) {
        if self.value_auto != value && !self.is_manual {
            self.has_changed = true;
        }
        self.value_auto = value;
    }

    pub fn manual_set(&mut self, value: T) {
        if (self.value_manual != value && self.is_manual)
            || (!self.is_manual && self.value_auto != value)
        {
            self.has_changed = true;
        }
        self.is_manual = true;
        self.value_manual = value;
    }

    pub fn resume_auto(&mut self) {
        if self.is_manual && self.value_manual != self.value_auto {
            self.has_changed = true;
        }
        self.is_manual = false;
    }

    pub fn get(&self) -> &T {
        if self.is_manual {
            &self.value_manual
        } else {
            &self.value_auto
        }
    }

    pub fn is_manual(&self) -> bool {
        return self.is_manual;
    }

    /// Checks if the internal value has changed since the last call to has_changed
    pub fn has_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            return true;
        }
        return false;
    }
}
