/* Rust functions exported for use in C */
struct rslib_ctx;

struct rslib_ctx *rslib_new(MPI_Comm);
void rslib_do_something(struct rslib_ctx *);
void rslib_free(struct rslib_ctx *);
