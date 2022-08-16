/// Rust library example with an exposed C interface.
use std::mem::MaybeUninit;

use mpi::ffi::{MPI_Comm, MPI_Comm_dup};
use mpi::topology::{Communicator, UserCommunicator};
use mpi::traits::*;

pub struct RSLibCtx {
    comm: UserCommunicator,
}

/// Create a new context, store the communicator and return it. Returns NULL on
/// failure.
#[no_mangle]
pub unsafe extern "C" fn rslib_new(comm: MPI_Comm) -> *mut RSLibCtx {
    // Duplicate the input communicator
    let mut mem = MaybeUninit::uninit();
    MPI_Comm_dup(comm, mem.as_mut_ptr());
    let lib_comm = mem.assume_init();
    if let Some(user_comm) = UserCommunicator::from_raw(lib_comm) {
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
pub unsafe extern "C" fn rslib_do_something(ctx: *mut RSLibCtx) {
    let ctx = Box::from_raw(ctx);
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
    Box::from_raw(ctx);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
