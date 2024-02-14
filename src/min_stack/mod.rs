#[derive(Default)]
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            let last_min = self.stack.last().unwrap().1;
            self.stack.push((val, i32::min(last_min, val)));
        }
    }

    pub fn pop(&mut self) {
        if !self.stack.is_empty() {
            self.stack.pop();
        }
    }

    pub fn top(&self) -> Option<i32> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.last().unwrap().0)
        }
    }

    pub fn get_min(&self) -> Option<i32> {
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.last().unwrap().1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_empty() {
        let mut stack = MinStack::default();
        stack.push(10);
        stack.pop();
        stack.pop();
        assert_eq!(stack.top(), None);
    }
}
