use mpi::ffi::MPI_Comm;
use mpi::topology::{AsCommunicator, UserCommunicator};

pub struct RSLibCtx {
    comm: UserCommunicator,
}

/// Create a new context, store the communicator and return it.
#[no_mangle]
pub extern "C" fn rslib_new(comm: MPI_Comm) -> *mut RSLibCtx {
    if let Some(user_comm) = unsafe { UserCommunicator::from_raw(comm) } {
        let ctx = Box::new(RSLibCtx {
            comm: user_comm,
        });
        // Get the raw pointer from the box
        Box::into_raw(ctx)
    } else {
        0 as *mut RSLibCtx
    }
}

/// Do something with MPI.
#[no_mangle]
pub extern "C" fn rslib_do_something(ctx: *mut RSLibCtx) {
    // TODO
}

/// Free the allocated context.
#[no_mangle]
pub unsafe extern "C" fn rslib_free(ctx: *mut RSLibCtx) {
    // Put the pointer back into the box
    let ctx = Box::from_raw(ctx);
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
