
use std::{alloc::System, collections::HashMap};

use tracking_allocator::{Allocator, AllocationTracker};

#[global_allocator]
static ALLOCATOR: Allocator<System> = Allocator::system();

#[derive(Debug, Clone, Default)]
pub(crate) struct MemoryStatistics {
    allocated: HashMap<usize, ()>,
}

impl AllocationTracker for MemoryStatistics {
    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: tracking_allocator::AllocationGroupId,
    ) {
        let mut i = 0;
        let mut matching_frames = Vec::with_capacity(4);
        backtrace::trace(|frame| {
            // let ip = frame.ip();
            // let symbol_address = frame.symbol_address();

            let mut name = None;
            let mut fname = None;
            let mut lineno = None;
            let mut colno = None;

            // Resolve this instruction pointer to a symbol name
            backtrace::resolve_frame(frame, |sym| {
                name = sym.name().map(|n| n.to_string());
                fname = sym.filename().map(|f| f.to_owned());
                lineno = sym.lineno();
                colno = sym.colno();
            });

            let cont = if let Some(func) = name {
                if func.contains("day") {
                    let label = match (fname, lineno, colno) {
                        (Some(file), Some(line), Some(col)) =>
                            format!("{func}@{}:{line:}:{col:}", file.display()),
                        (Some(file), Some(line), _) =>
                            format!("{func}@{}:{line:}", file.display()),
                        (Some(file), _, _) =>
                            format!("{func}@{}", file.display()),
                        (_, _, _) =>
                            format!("{func}"),
                    };

                    matching_frames.push((i, func, format!("memory allocation @{:p} (frame idx[{}]) {}", frame.ip(), i, label)));
                    true
                } else {
                    true
                }
            } else {
                true
            };

            i += 1;
            cont // keep going to the next frame
        });

        matching_frames.iter()
            .enumerate()
            .for_each(|(li, (_fi, _func, logmsg))| {
                eprintln!("[{}] {}", li, logmsg);
            });
    }
    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: tracking_allocator::AllocationGroupId,
        current_group_id: tracking_allocator::AllocationGroupId,
    ) {

    }
}
