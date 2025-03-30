pub struct Slot<T, const N: usize> {
    array: [Option<T>; N],
    size: u8,
}

impl<T, const N: usize> Slot<T, N> {
    pub fn new() -> Self {
        Self {
            array: [const { None }; N],
            size: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.size
    }

    // returns index of the pins
    pub fn add(&mut self, pin: T) -> Option<usize> {
        if self.size >= 16 {
            return None;
        }
        for i in 0..self.array.len() {
            if self.array[i].is_none() {
                self.array[i] = Some(pin);
                assert_eq!(self.size as usize, i);
                self.size += 1;
                return Some(i);
            }
        }
        unreachable!();
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index >= self.array.len() {
            return false;
        }
        let old = self.array[index].take();
        if old.is_none() {
            return false;
        }
        assert!(self.size > 0);
        self.size -= 1;
        true
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.array.len() {
            return None;
        }
        self.array[index].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.array.len() {
            return None;
        }
        self.array[index].as_mut()
    }
}
