/* Simple Rust to C example */
#include <stdio.h>
#include <stdlib.h>
#include <mpi.h>

#include "mpi_lib/src/rust.h"

int main(int argc, char *argv[])
{
    int rank;
    int size;
    struct rslib_ctx *ctx;
    int ret = 0;

    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    ctx = rslib_new(MPI_COMM_WORLD);
    if (ctx == NULL) {
        fprintf(stderr, "rslib_new() failed");
        ret = 1;
        goto out;
    }
    rslib_do_something(ctx);
    // Simple barrier outside the Rust code
    printf("C BARRIER (rank %i)\n", rank);
    MPI_Barrier(MPI_COMM_WORLD);
    rslib_free(ctx);

out:
    MPI_Finalize();
    return ret;
}
