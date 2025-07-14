use core::fmt::Debug;

type DynFn = dyn Fn();

#[derive(Clone, Copy)]
pub struct Task<'a> {
    pub f: &'a DynFn,
}

impl<'a> Debug for Task<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "task")
    }
}

#[derive(Clone, Copy)]
pub struct Coaster<'a, const S: usize> {
    queue: [Option<Task<'a>>; S],
    last: usize,
}

const NONE: Option<Task> = None;

impl<'a, const S: usize> Coaster<'a, S> {
    pub fn new() -> Coaster<'a, S> {
        let queue = [NONE; S];
        let c = Coaster { queue, last: 0 };
        c
    }

    /// Add a task to the queue if a free slot can be found.
    pub fn add(&mut self, t: Task<'a>) -> Result<usize, ()> {
        for (i, e) in self.queue.iter().enumerate() {
            if e.is_none() {
                self.queue[i] = Some(t);
                return Ok(i);
            }
        }
        Err(())
    }

    /// Advance the task ID for the next task.
    fn next(&mut self) {
        self.last += 1;
        // Ensure to remain within the queue and within existing tasks.
        // Note that the queue may have no task at all.
        // In that case, we remian at the start.
        if self.last >= S || self.queue[self.last].is_none() {
            self.last = 0;
        }
    }

    /// Advance one step in the queue, if a task is available.
    pub fn step(self: &mut Self) -> Option<usize> {
        let last = self.last;
        let task_id = match self.queue[last] {
            Some(t) => {
                let f = t.f;
                f();
                Some(last)
            }
            // We may have no tasks in the queue.
            None => None,
        };
        self.next();
        task_id
    }

    /// Run queue to the end.
    pub fn run(self: &mut Self) {
        for e in self.queue[self.last..].iter() {
            if let Some(t) = e {
                let f = t.f;
                f();
            } else {
                break;
            }
        }
        self.last = 0;
    }
}

impl<'a, const S: usize> Debug for Coaster<'a, S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let _ = write!(f, "Coaster");
        for e in self.queue {
            let _ = write!(f, "\n  ");
            match e {
                Some(t) => {
                    let _ = write!(f, "{t:?}");
                }
                None => {
                    let _ = write!(f, "[no task]");
                }
            }
        }
        write!(f, "")
    }
}
