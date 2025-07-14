use lib::coaster::{Coaster, Task};
use lib::wheel::Wheel;

fn do_it() {
    println!("do_it");
}

fn foodeloo() {
    println!("foodeloo");
}

fn funsiez() {
    println!("funsiez");
}

const SIZE: usize = 8;

fn turn(w: &mut Wheel<SIZE>) {
    if let Some(task_id) = w.turn() {
        println!("  task {task_id:?} ");
    } else {
        println!("  [no task]");
    }
}

fn add_task<'a, 's>(c: &'s mut Coaster<'a, SIZE>, t: Task<'a>) {
    match c.add(t) {
        Ok(task_id) => println!("  new task {task_id}"),
        _ => println!("  cannot add task"),
    }
}

fn main() {
    println!("Create new Coaster 🎢 with {SIZE} slots");
    let mut c: Coaster<SIZE> = Coaster::<SIZE>::new();
    println!("{c:#?}");

    println!("\nMake a Wheel 🎡 and turn");
    let mut w = Wheel::new(c);
    println!("{w:#?}");

    for _ in 0..2 {
        turn(&mut w);
    }

    println!("\nAdd tasks to the Coaster 🎢");
    for i in 0..4 {
        let t = if i % 2 == 0 {
            Task { f: &foodeloo }
        } else {
            Task { f: &do_it }
        };
        add_task(&mut c, t);
    }
    println!("{c:#?}");

    println!("\nMake a new Wheel 🎡 and turn");
    let mut w = Wheel::new(c);
    println!("{w:#?}");

    for _ in 0..5 {
        turn(&mut w);
    }
    println!("\nRotate the Wheel 🎡");
    w.rotate();

    println!("\nAdd some more tasks");
    for i in 0..6 {
        let t = if i % 2 == 0 {
            Task { f: &funsiez }
        } else {
            Task { f: &do_it }
        };
        add_task(&mut c, t);
    }
    println!("{c:#?}");

    println!("\nMake another Wheel 🎡 and turn");
    let mut w = Wheel::new(c);

    println!("{w:#?}");
    for _ in 0..5 {
        turn(&mut w);
    }
    println!("\nRotate the Wheel 🎡");
    w.rotate();
}
