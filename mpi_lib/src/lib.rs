use mpi::ffi::MPI_Comm;
use mpi::topology::{Communicator, UserCommunicator};
use mpi::traits::*;

pub struct RSLibCtx {
    comm: UserCommunicator,
}

/// Create a new context, store the communicator and return it. Returns NULL on
/// failure.
#[no_mangle]
pub extern "C" fn rslib_new(comm: MPI_Comm) -> *mut RSLibCtx {
    if let Some(user_comm) = unsafe { UserCommunicator::from_raw(comm) } {
        let ctx = Box::new(RSLibCtx {
            comm: user_comm,
        });
        // Get the raw pointer from the box
        Box::into_raw(ctx)
    } else {
        std::ptr::null_mut()
    }
}

/// Do something with MPI.
#[no_mangle]
pub extern "C" fn rslib_do_something(ctx: *mut RSLibCtx) {
    let ctx = unsafe { Box::from_raw(ctx) };
    let rank = ctx.comm.rank();
    // Just do a simple MPI_Barrier()
    println!("RUST BARRIER (rank {})", rank);
    ctx.comm.barrier();
    // The below call is needed to avoid freeing the boxed memory
    Box::into_raw(ctx);
}

/// Free the allocated context.
#[no_mangle]
pub unsafe extern "C" fn rslib_free(ctx: *mut RSLibCtx) {
    // Put the pointer back into the box
    let ctx = Box::from_raw(ctx);
    // Don't free the communicator
    std::mem::forget(ctx.comm);
    // The box will now be dropped (and freed)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
