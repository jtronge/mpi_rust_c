MPI_LIB = mpi_lib/target/release/libmpi_lib.a
CFLAGS = -g
CC = mpicc
LDFLAGS = -ldl

main: main.o $(MPI_LIB)
	$(CC) -o $@ $^ $(LDFLAGS)

$(MPI_LIB): mpi_lib/src/lib.rs
	cd mpi_lib && cargo build --release
