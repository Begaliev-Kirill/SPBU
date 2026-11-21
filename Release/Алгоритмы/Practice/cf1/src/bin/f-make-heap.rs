use std::io;
use std::cmp::Ord;
//use std::error::Error;

struct MaxHeap<T>
where
    T : Ord,
{
    data : Vec<T>,
}

impl<T> MaxHeap<T> 
where
    T: Ord,
{
    fn new() -> Self {
        MaxHeap {
            data : Vec::new()
        }
    }

    fn sift_up(&mut self, mut index : usize) {
        while index != 0 {
            if self.data[index] > self.data[(index-1)/2] {
                self.data.swap(index, (index-1)/2);
                index = (index-1)/2;
                continue
            }
            break
        }
    }
    
    fn sift_down(&mut self, mut index : usize) {
        let n : usize =  self.data.len();

        while index*2 + 1 < n {
            let left_child_index : usize = index*2 + 1;
            let right_child_index : usize = index*2 + 2;
            let mut max_child_index : usize = left_child_index;
         
            if right_child_index < n  
               && self.data[left_child_index] < self.data[right_child_index] {
               max_child_index = right_child_index; 
            }

            if self.data[index] < self.data[max_child_index] {
                self.data.swap(index, max_child_index);
                index = max_child_index;
                continue
            }

            break 
        }
    }

    fn insert(&mut self, value : T) {
        let n : usize = self.data.len();
        self.data.push(value);
        self.sift_up(n);
    }
   
    fn extract(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let last_index : usize = self.data.len()-1;
        self.data.swap(0, last_index);
        let result : T = self.data.pop().expect("Always Some"); 

        self.sift_down(0);

        Some(result)
    }
}

//io::stdout().flush()?; //copy paste for buffer clean
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let count : usize = input
        .trim()
        .parse()
        .expect("not a number");
    input.clear();

    let mut max_heap : MaxHeap<u32> = MaxHeap::new();
    
    for _ in 0..count {
        io::stdin()
            .read_line(&mut input)
            .expect("input 2 error");
        
        let mut variables = input.split_whitespace();
        let command : bool = variables.next().unwrap() == "1";
        let value : Option<u32> = variables.next().map(|x| x.parse().unwrap());
        input.clear();

        if command {
            let result = max_heap.extract().unwrap();
            println!("{result}");
        } else {
            if let Some(x) = value {
                max_heap.insert(x);
            }
        }
    }
}
