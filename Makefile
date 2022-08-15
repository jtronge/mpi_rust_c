MPI_LIB = mpi_lib/target/release/libmpi_lib.a
CC = mpicc

main: main.o $(MPI_LIB)
	$(CC) -o $@ $^

$(MPI_LIB): mpi_lib/src/lib.rs
	cd mpi_lib && cargo build --release
