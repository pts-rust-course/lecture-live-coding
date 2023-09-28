struct SegmentSumComputer {
    elements: Vec<i32>,
    /// prefix_sum[i] = sum(elements[0..i))
    prefix_sum: Vec<i32>
}

impl SegmentSumComputer {
    fn new() -> Self {
        SegmentSumComputer {
            elements: Vec::new(),
            prefix_sum: vec![0],
        }
    }

    fn push(&mut self, element: i32) {
        self.elements.push(element);
        self.prefix_sum.push(
            *self.prefix_sum.last().unwrap() + element
        );
    }

    fn pop(&mut self) -> Option<i32> {
        let res = self.elements.pop()?;
        self.prefix_sum.pop();

        Some(res)
    }

    /// Returns sum(elements[from..to))
    fn sum(&self, from: usize, to: usize) -> Option<i32> {
        if !(0..=self.elements.len()).contains(&to)
           || from > to
        {
            return None;
        }

        Some(self.prefix_sum[to] - self.prefix_sum[from])
    }
}

fn main() {
    let mut s = SegmentSumComputer::new();

    println!("{:?}", s.sum(0, 0));

    s.push(1);
    s.push(2);
    s.push(-3);

    println!("{:?}", s.sum(0, 1));
    println!("{:?}", s.sum(0, 2));
    println!("{:?}", s.sum(0, 3));

    println!("{:?}", s.sum(1, 0));

    println!("{:?}", s.sum(1, 3));

    println!("{:?}", s.sum(1, 1000));

    println!("First pop: {:?}", s.pop());
    println!("Second pop: {:?}", s.pop());
    println!("Third pop: {:?}", s.pop());
    println!("Fourth pop: {:?}", s.pop());
    println!("Fifth pop: {:?}", s.pop());

    s.push(1);
    s.push(2);
    s.push(-3);

    println!("{:?}", s.sum(0, 1));
    println!("{:?}", s.sum(0, 2));
    println!("{:?}", s.sum(0, 3));

    println!("{:?}", s.sum(1, 0));

    println!("{:?}", s.sum(1, 3));

    println!("{:?}", s.sum(1, 1000));

}
