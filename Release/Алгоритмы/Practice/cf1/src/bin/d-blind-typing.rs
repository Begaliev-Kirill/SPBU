use std::io::{self, Write};
use std::error::Error;

//io::stdout().flush()?; //copy paste for buffer clean
fn main () -> Result<(), Box<dyn Error>> {
   let mut input = String::new();
   io::stdin()
       .read_line(&mut input)?;
    
   let v: Vec<i64> = input
       .split_whitespace()
       .map(|c| c.parse())
       .collect::<Result<_, _>>()?;    
   let n = v[0] as usize;
   let mut q_count = v[1] as i32;
   
   let mut answer : Vec<usize> = vec![0;n];
   let sorted_indices : Vec<usize> = indices_merge_sort(&mut (1..=n).collect())?;

   for i in 0..n {
       answer[sorted_indices[i]-1] = i + 1;
   }
   let mut str_answer : String = String::with_capacity(n);
   for element in answer.iter() {
       str_answer.push_str(&format!("{element} "));
   }
   str_answer.pop();
   
   println!("! {str_answer}");
   io::stdout().flush()?;
   input.clear();
   io::stdin()
       .read_line(&mut input);

   Ok(())
}

fn indices_merge_sort(nums : &mut Vec<usize>) -> Result<Vec<usize>, Box<dyn Error>> {
    io::stdout().flush()?;
    let n : usize = nums.len();
    let mut input = String::new();

    match n {
        0 => {return Ok(nums.to_vec())},
        1 => {return Ok(nums.to_vec())},
        2 => {
            println!("+ {}",nums[0]);
            io::stdin()
                .read_line(&mut input);
            
            io::stdout().flush()?;
            input.clear();
            println!("+ {}",nums[1]);
            io::stdin()
                .read_line(&mut input);

            if input.trim() == "YES" {nums.swap(0,1);}
            return Ok(nums.to_vec());
        },
        _ => {}
    }

    let mid : usize = n/2;
    let (left,right) = nums.split_at_mut(mid);
    let mut left = indices_merge_sort(&mut left.to_vec())?;
    let mut right = indices_merge_sort(&mut right.to_vec())?;

    let mut result : Vec<usize> = Vec::with_capacity(n);
    println!("+ {}",left[0]);
    io::stdin()
        .read_line(&mut input);
    io::stdout().flush()?;

    let (mut i, mut j) : (usize, usize) = (0,0);
    let mut switched_pointer : bool = false;
    while i < left.len() && j < right.len() {
        input.clear();
        if !switched_pointer {
            println!("+ {}",right[j]);
        } else {
            println!("+ {}",left[i]);
        }
        io::stdout().flush()?;
        
        io::stdin()
            .read_line(&mut input);
        if input.trim() == "YES" {
            if !switched_pointer {
                result.push(right[j]);
                j += 1;

                println!("+ {}",left[i]);
                io::stdout().flush()?;

                input.clear();
                io::stdin()
                    .read_line(&mut input);
            } 
            else {
                result.push(left[i]);
                i += 1;

                println!("+ {}",right[j]);
                io::stdout().flush()?;

                input.clear();
                io::stdin()
                    .read_line(&mut input);
            }
        } 
        else {
            if input.trim() == "FAIL" {
                return Ok(nums.to_vec());
            }

            if !switched_pointer {
                result.push(left[i]);
                i += 1;
                switched_pointer = !switched_pointer;
            }
            else {
                result.push(right[j]);
                j += 1;
                switched_pointer = !switched_pointer;
            }
        }
    }

    let mut left_tail = left.split_off(i);
    let mut right_tail = right.split_off(j);
    
    result.append(&mut left_tail);
    result.append(&mut right_tail);

    Ok(result)
}
