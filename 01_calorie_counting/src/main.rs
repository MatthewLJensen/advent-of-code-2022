use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    println!("Elf Input");
    let mut guess = String::new();

    let num_top_elves = 3; // Change this to get a varying number of top elves
    let mut elf_heap = BinaryHeap::new(); // Effectively a min heap

    let mut current_total = 0;
    let mut end_on_next_empty_input = false;
    loop {
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

        guess = guess.trim().to_string();

        if guess == "" {
            if end_on_next_empty_input {
                break;
            };
            end_on_next_empty_input = true;

            elf_heap.push(Reverse(current_total));
            if elf_heap.len() > num_top_elves {
                elf_heap.pop();
            };
            
            current_total = 0;
            continue;
        } else { end_on_next_empty_input = false; }

        current_total += guess.parse::<i32>().unwrap();
    }

    let result: i32 = elf_heap.into_iter().map(|i| i.0).sum();

    println!("Final Total: {}", result);
}