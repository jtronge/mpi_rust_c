/* Simpl Rust to C example */
#include <mpi.h>

#include "mpi_lib/src/rust.h"

int main(int argc, char *argv[])
{
    int rank;
    int size;
    struct rslib_ctx *ctx;

    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    ctx = rslib_new(MPI_COMM_WORLD);
    rslib_do_something(ctx);
    rslib_free(ctx);

    MPI_Finalize();
}
