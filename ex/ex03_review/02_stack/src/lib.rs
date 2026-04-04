#[derive(Debug, Default)]
pub struct Stack {
    #[allow(dead_code)]
    items: Vec<i32>,
}

impl Stack {
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("空の `Stack` を返してください");
    }

    #[allow(unused_variables)]
    pub fn push(&mut self, value: i32) {
        todo!("`value` をスタックに積んでください");
    }

    #[allow(unused_variables)]
    pub fn pop(&mut self) -> Option<i32> {
        todo!("一番上の要素を取り出してください。空なら `None` です");
    }

    #[allow(unused_variables)]
    pub fn peek(&self) -> Option<&i32> {
        todo!("一番上の要素への参照を返してください。空なら `None` です");
    }

    #[allow(unused_variables)]
    pub fn len(&self) -> usize {
        todo!("現在の要素数を返してください");
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn new_stack_starts_empty() {
        let stack = Stack::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn push_and_pop_follow_lifo() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peek_does_not_remove_top_value() {
        let mut stack = Stack::new();
        stack.push(42);

        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(42));
    }
}

