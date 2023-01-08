use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Instant,
};

static INPUT: &str = include_str!("input.txt");

static BUCKET_SIZE: usize = 100;

#[derive(Debug)]
struct Element<T> {
    pub prev: Option<Rc<RefCell<Element<T>>>>,
    pub next: Option<Rc<RefCell<Element<T>>>>,
    pub bucket: Option<Weak<RefCell<Bucket<T>>>>,
    pub value: T,
}

impl<T> Element<T> {
    pub fn new(value: T) -> Self {
        Self {
            prev: None,
            next: None,
            bucket: None,
            value,
        }
    }

    pub fn bucket(&self) -> Rc<RefCell<Bucket<T>>> {
        self.bucket.as_ref().unwrap().upgrade().unwrap()
    }
}

#[derive(Debug)]
struct Bucket<T> {
    pub offset: usize,
    pub idx: usize,
    pub head: Rc<RefCell<Element<T>>>,
    pub tail: Rc<RefCell<Element<T>>>,
}

impl<T> Bucket<T> {
    // pub fn len(&self) -> usize {
    //     let mut len = 0;

    //     let mut current = Rc::clone(&self.head);
    //     loop {
    //         len += 1;
    //         let next = current.borrow().next.as_ref().map(Rc::clone);
    //         match next {
    //             Some(next) => current = Rc::clone(&next),
    //             None => break,
    //         }
    //     }

    //     len
    // }

    /**
     * Simply connects the previous neighbors.
     */
    pub fn remove(&mut self, element: &Rc<RefCell<Element<T>>>) {
        let element = element.borrow_mut();

        // Next -> Prev
        if let Some(next) = &element.next {
            match &element.prev {
                None => {
                    self.head = Rc::clone(next);
                    next.borrow_mut().prev = None;
                }
                Some(prev) => {
                    next.borrow_mut().prev = Some(Rc::clone(prev));
                }
            }
        }

        // Prev -> Next
        if let Some(prev) = &element.prev {
            match &element.next {
                None => {
                    self.tail = Rc::clone(prev);
                    prev.borrow_mut().next = None;
                }
                Some(next) => {
                    prev.borrow_mut().next = Some(Rc::clone(next));
                }
            }
        }
    }

    pub fn pop_front(&mut self) -> Rc<RefCell<Element<T>>> {
        let head = Rc::clone(&self.head);

        match head.borrow().next.as_ref() {
            None => panic!(),
            Some(next) => {
                next.borrow_mut().prev = None;
                self.head = Rc::clone(next);
            }
        }

        head
    }

    pub fn pop_back(&mut self) -> Rc<RefCell<Element<T>>> {
        let tail = Rc::clone(&self.tail);

        match tail.borrow().prev.as_ref() {
            None => panic!(),
            Some(prev) => {
                prev.borrow_mut().next = None;
                self.tail = Rc::clone(prev);
            }
        }

        tail
    }

    pub fn push_front(&mut self, element: &Rc<RefCell<Element<T>>>) {
        self.head.borrow_mut().prev = Some(Rc::clone(element));
        element.borrow_mut().prev = None;
        element.borrow_mut().next = Some(Rc::clone(&self.head));
        self.head = Rc::clone(element);
    }

    pub fn push_back(&mut self, element: &Rc<RefCell<Element<T>>>) {
        self.tail.borrow_mut().next = Some(Rc::clone(element));
        element.borrow_mut().next = None;
        element.borrow_mut().prev = Some(Rc::clone(&self.tail));
        self.tail = Rc::clone(element);
    }

    /**
     * `element` is inserted right of `insert_at`
     */
    pub fn insert_at_element(
        &mut self,
        element: &Rc<RefCell<Element<T>>>,
        insert_at: &Rc<RefCell<Element<T>>>,
    ) {
        let mut current = element.borrow_mut();

        // Current -> Swap.Next
        // Swap.Next -> Current
        match &insert_at.borrow().next {
            None => {
                current.next = None;
                self.tail = Rc::clone(element);
            }
            Some(next) => {
                next.borrow_mut().prev = Some(Rc::clone(element));
                current.next = Some(Rc::clone(next));
            }
        }

        // Connect Swap -> Current
        current.prev = Some(Rc::clone(insert_at));
        insert_at.borrow_mut().next = Some(Rc::clone(element));
    }

    pub fn element_at_offset_from_front(
        element: &Rc<RefCell<Element<T>>>,
        initial: usize,
    ) -> Option<Rc<RefCell<Element<T>>>> {
        let mut offset = initial;

        let mut current = Rc::clone(element);
        loop {
            if offset == 0 {
                return Some(current);
            }

            let next = current.borrow().next.as_ref().map(Rc::clone);
            match next {
                Some(next) => current = Rc::clone(&next),
                None => return None,
            }

            offset -= 1;
        }
    }

    pub fn element_at_offset_from_back(
        element: &Rc<RefCell<Element<T>>>,
        offset: usize,
    ) -> Option<Rc<RefCell<Element<T>>>> {
        let mut offset = offset;

        let mut current = Rc::clone(element);
        loop {
            if offset == 0 {
                return Some(current);
            }

            let prev = current.borrow().prev.as_ref().map(Rc::clone);
            match prev {
                Some(prev) => current = Rc::clone(&prev),
                None => return None,
            }

            offset -= 1;
        }
    }

    pub fn insert_at(&mut self, element: &Rc<RefCell<Element<T>>>, position: usize) {
        let target = if position < BUCKET_SIZE / 2 {
            Bucket::element_at_offset_from_front(&self.head, position).unwrap()
        } else {
            Bucket::element_at_offset_from_back(&self.tail, BUCKET_SIZE - 1 - position).unwrap()
        };

        self.insert_at_element(element, &target);
    }
}

impl<T: std::fmt::Debug> Bucket<T> {
    pub fn display(&self) -> String {
        let mut next = Some(Rc::clone(&self.head));
        let mut output = String::new();
        loop {
            match next {
                None => break,
                Some(current) => {
                    output = format!("{output}{:?} ", current.borrow().value);
                    next = current.borrow().next.as_ref().map(Rc::clone);
                }
            }
        }

        output
    }
}

struct ShiftList {
    pub bucket_size: usize,
    pub len: i32,
    pub buckets: Vec<Rc<RefCell<Bucket<i32>>>>,
    pub all_elements: Vec<Rc<RefCell<Element<i32>>>>,
}

impl ShiftList {
    pub fn new(input: &str, bucket_size: usize) -> Self {
        let values: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
        let value_amount = values.len() as i32;
        assert!(value_amount % BUCKET_SIZE as i32 == 0);

        let mut buckets = Vec::with_capacity((values.len() / BUCKET_SIZE) + 1);
        let mut all_elements = Vec::with_capacity(values.len());

        for (i, chunk) in values.chunks(BUCKET_SIZE).enumerate() {
            // TODO: Do this initially?
            let elements: Vec<_> = chunk
                .iter()
                .copied()
                .map(Element::new)
                .map(RefCell::new)
                .map(Rc::new)
                .collect();

            let bucket = Rc::new(RefCell::new(Bucket {
                offset: i * BUCKET_SIZE,
                idx: i,
                head: Rc::clone(&elements[0]),
                tail: Rc::clone(&elements[elements.len() - 1]),
            }));

            elements[0].borrow_mut().bucket = Some(Rc::downgrade(&bucket));

            for j in 1..elements.len() {
                let mut element = elements[j].borrow_mut();
                element.bucket = Some(Rc::downgrade(&bucket));
                element.prev = Some(Rc::clone(&elements[j - 1]));
            }

            for j in 0..(elements.len() - 1) {
                let mut element = elements[j].borrow_mut();
                element.next = Some(Rc::clone(&elements[j + 1]));
            }

            all_elements.extend(elements);
            buckets.push(bucket);
        }

        ShiftList {
            bucket_size,
            buckets,
            len: all_elements.len() as i32,
            all_elements,
        }
    }

    pub fn shift(&self) {
        for element in &self.all_elements {
            let (global_origin_idx, global_target_idx) = self.get_origin_and_target_idx(element);

            if global_origin_idx == global_target_idx {
                continue;
            }

            let target_bucket_idx = global_target_idx as usize / self.bucket_size;
            let local_target_idx = global_target_idx as usize % self.bucket_size;

            let origin_bucket = element.borrow().bucket();
            let mut origin_bucket = origin_bucket.borrow_mut();

            // If move happens in same bucket, just move inside of bucket
            if origin_bucket.idx == target_bucket_idx {
                let old_prev = element.borrow_mut().prev.as_ref().map(Rc::clone);
                let old_next = element.borrow_mut().next.as_ref().map(Rc::clone);

                origin_bucket.insert_at(element, local_target_idx);

                if let Some(next) = &old_next {
                    match &old_prev {
                        None => {
                            origin_bucket.head = Rc::clone(next);
                            next.borrow_mut().prev = None;
                        }
                        Some(prev) => next.borrow_mut().prev = Some(Rc::clone(prev)),
                    }
                }
                if let Some(prev) = &old_prev {
                    match &old_next {
                        None => {
                            origin_bucket.tail = Rc::clone(prev);
                            prev.borrow_mut().next = None;
                        }
                        Some(next) => {
                            prev.borrow_mut().next = Some(Rc::clone(next));
                        }
                    }
                }

                continue;
            }

            origin_bucket.remove(element);

            self.buckets[target_bucket_idx]
                .borrow_mut()
                .insert_at(element, local_target_idx);

            element.borrow_mut().bucket = Some(Rc::downgrade(&self.buckets[target_bucket_idx]));

            // TODO: Beautify
            // Re-Adjustment

            // We check which move direction would result in less operations.
            let distance_to_target_left = if global_target_idx < global_origin_idx {
                global_origin_idx - global_target_idx
            } else {
                global_origin_idx + (self.len - global_target_idx)
            };

            let distance_to_target_right = if global_target_idx > global_origin_idx {
                global_target_idx - global_origin_idx
            } else {
                (self.len - global_origin_idx) + global_target_idx
            };

            if distance_to_target_left < distance_to_target_right {
                let mut idx = target_bucket_idx;
                loop {
                    let back = self.buckets[idx].borrow_mut().pop_back();

                    // Wrap around
                    let next_idx = if idx == self.buckets.len() - 1 {
                        0
                    } else {
                        idx + 1
                    };

                    if next_idx == origin_bucket.idx {
                        origin_bucket.push_front(&back);
                        back.borrow_mut().bucket = Some(Rc::downgrade(&self.buckets[next_idx]));
                        break;
                    }

                    self.buckets[next_idx].borrow_mut().push_front(&back);
                    back.borrow_mut().bucket = Some(Rc::downgrade(&self.buckets[next_idx]));

                    idx = next_idx;
                }
            } else {
                let mut idx = target_bucket_idx;
                loop {
                    let front = self.buckets[idx].borrow_mut().pop_front();

                    // Wrap around
                    let next_idx = if idx == 0 {
                        self.buckets.len() - 1
                    } else {
                        idx - 1
                    };

                    if next_idx == origin_bucket.idx {
                        origin_bucket.push_back(&front);
                        front.borrow_mut().bucket = Some(Rc::downgrade(&self.buckets[next_idx]));
                        break;
                    }

                    self.buckets[next_idx].borrow_mut().push_back(&front);
                    front.borrow_mut().bucket = Some(Rc::downgrade(&self.buckets[next_idx]));

                    idx = next_idx;
                }
            }
        }
    }

    fn get_origin_and_target_idx(&self, element: &Rc<RefCell<Element<i32>>>) -> (i32, i32) {
        let local_origin_idx = {
            let mut distance_to_head = 0;
            let mut current = Rc::clone(element);
            loop {
                let prev = current.borrow().prev.as_ref().map(Rc::clone);
                match prev {
                    Some(prev) => current = prev,
                    None => break,
                };
                distance_to_head += 1;
            }
            distance_to_head
        };

        let element = element.borrow();
        let global_origin_idx = (element.bucket().borrow().offset + local_origin_idx) as i32;

        // We can skip all the moves that would wrap one (ore more) complete round.
        let moves = element.value.abs() % self.len;

        let mut target_index = if element.value < 0 {
            // TODO: Explain
            global_origin_idx - moves - 1
        } else {
            global_origin_idx + moves
        };

        target_index %= self.len;
        if target_index < 0 {
            target_index = self.len - target_index.abs()
        }

        (global_origin_idx, target_index)
    }

    pub fn get_value_at_offset(&self, start: &Rc<RefCell<Element<i32>>>, offset: usize) -> i32 {
        // Find offset until end of bucket
        let mut counter = 0;
        let mut current = Rc::clone(start);
        loop {
            let next = current.borrow().next.as_ref().map(Rc::clone);
            match next {
                Some(next) => current = next,
                None => break,
            }

            counter += 1;
        }

        // Calculate target with the rest
        let offset = offset - counter;

        let origin_bucket_idx = start.borrow().bucket().borrow().idx;

        let buckets_traversed = offset / BUCKET_SIZE;
        let target_local_index = (offset % BUCKET_SIZE) - 1;
        let buckets_offset = buckets_traversed % self.buckets.len();

        let target_bucket_idx = (origin_bucket_idx + buckets_offset) % self.buckets.len();
        let target_element = Bucket::element_at_offset_from_front(
            &self.buckets[target_bucket_idx].borrow().head,
            target_local_index,
        );

        target_element.unwrap().borrow().value
    }
}

/**
 * TODO:
 * - Don't borrow so much?
 * - Some unnecessary copies?
 */
pub fn part_one() {
    let shift_list = ShiftList::new(INPUT, BUCKET_SIZE);

    let zero = shift_list
        .all_elements
        .iter()
        .find(|element| element.borrow().value == 0)
        .map(Rc::clone)
        .unwrap();

    shift_list.shift();

    let mut sum = 0;
    for offset in [1000, 2000, 3000] {
        sum += shift_list.get_value_at_offset(&zero, offset);
    }

    println!("{sum}");
}

// #[cfg(test)]
// mod tests {
//     use super::{part_one, INPUT};

//     #[test]
//     fn test_part_one() {
//         assert_eq!(part_one(), 3)
//     }

//     // test-2: 0 3 1 + -2 2 -3 +
// }
