/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // self.q1.enqueue(elem);

        // Move all elements from q1 to q2
        while let Ok(value) = self.q1.dequeue() {
            self.q2.enqueue(value);
        }

        // Enqueue the new element to q1
        self.q1.enqueue(elem);

        // Move all elements back from q2 to q1
        while let Ok(value) = self.q2.dequeue() {
            self.q1.enqueue(value);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // // 如果q1为空，返回错误
        // if self.q1.is_empty() {
        //     return Err("Stack is empty");
        // }

        // // 如果q1中只有一个元素，直接出队返回即可
        // if self.q1.size() == 1 {
        //     return self.q1.dequeue();
        // }

        // // 将q1中的元素依次出队并入队到q2，直到q1中只剩下一个元素
        // while self.q1.size() > 1 {
        //     if let Ok(elem) = self.q1.dequeue() {
        //         self.q2.enqueue(elem);
        //     }
        // }

        // // 将q1中剩下的最后一个元素出队，即为栈顶元素
        // let result = self.q1.dequeue().unwrap(); // 在这里使用 unwrap 是安全的，因为上面已经确保了 q1 不为空

        // // 交换 q1 和 q2 的引用，使得 q1 始终为空队列
        // std::mem::swap(&mut self.q1, &mut self.q2);

        // // 返回栈顶元素
        // Ok(result)

        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
		self.q1.dequeue()
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}