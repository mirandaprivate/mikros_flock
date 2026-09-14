#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <flint/flint.h>
#include <flint/fq_nmod.h>
#include <flint/nmod_poly.h>
#include <flint/nmod_poly_factor.h>
#include <flint/ulong_extras.h>
#include <mpfr.h>

/* FLINT public API throughput, not a custom extension multiplication kernel. */
enum { BATCH = 256, MAX_DEGREE = 16, NFIELDS = 4, NOPS = 7 };
enum { ADD, SUB, MUL, SQUARE, SCALE, INVERSE, FOLD };
static const char *op_names[NOPS] = {
    "add", "sub", "mul", "square", "scale", "inverse", "fold"
};
typedef struct {
    const char *name;
    uint64_t p, nonresidue;
    int degree;
    fq_nmod_ctx_t ctx;
    fq_nmod_struct a[BATCH], b[BATCH], out[BATCH];
    fq_nmod_t challenge, tmp1, tmp2;
    ulong scalars[BATCH];
    uint64_t expected[NOPS][BATCH][MAX_DEGREE];
    uint64_t loops[NOPS];
    double context_seconds;
} profile;
static profile profiles[NFIELDS] = {
    {.name="F241", .p=241, .nonresidue=7, .degree=16},
    {.name="F61441", .p=61441, .nonresidue=11, .degree=8},
    {.name="KoalaBear", .p=UINT64_C(2130706433), .nonresidue=3, .degree=4},
    {.name="Goldilocks", .p=UINT64_C(18446744069414584321), .nonresidue=7, .degree=2}
};
static volatile uint64_t sink;
static uint64_t checks;

static void fail(const char *message) {
    fprintf(stderr, "FAIL: %s\n", message);
    exit(1);
}
static double seconds(clockid_t clock_id) {
    struct timespec t;
    if (clock_gettime(clock_id, &t)) fail("clock_gettime");
    return (double)t.tv_sec + (double)t.tv_nsec * 1e-9;
}
static uint64_t random_word(uint64_t *s) {
    uint64_t z = (*s += UINT64_C(0x9e3779b97f4a7c15));
    z = (z ^ (z >> 30)) * UINT64_C(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)) * UINT64_C(0x94d049bb133111eb);
    return z ^ (z >> 31);
}
static uint64_t random_mod(uint64_t *s, uint64_t p) {
    uint64_t v, threshold = (uint64_t)(-p) % p;
    do { v = random_word(s); } while (v < threshold);
    return v % p;
}

/* Independent schoolbook oracle: ordinary integer arithmetic, no FLINT ops. */
static uint64_t madd(uint64_t a, uint64_t b, uint64_t p) {
    return (uint64_t)(((__uint128_t)a + b) % p);
}
static uint64_t msub(uint64_t a, uint64_t b, uint64_t p) {
    return a >= b ? a - b : p - (b - a);
}
static uint64_t mmul(uint64_t a, uint64_t b, uint64_t p) {
    return (uint64_t)((__uint128_t)a * b % p);
}
static void oracle_mul(uint64_t *z, const uint64_t *a, const uint64_t *b,
                       const profile *f) {
    uint64_t c[2 * MAX_DEGREE] = {0};
    for (int i = 0; i < f->degree; i++)
        for (int j = 0; j < f->degree; j++)
            c[i+j] = madd(c[i+j], mmul(a[i], b[j], f->p), f->p);
    for (int i = 2 * f->degree - 2; i >= f->degree; i--)
        c[i-f->degree] = madd(c[i-f->degree],
                              mmul(c[i], f->nonresidue, f->p), f->p);
    memcpy(z, c, (size_t)f->degree * sizeof(uint64_t));
}
static void read_coeffs(uint64_t *a, const fq_nmod_t x, const profile *f) {
    for (int k = 0; k < f->degree; k++)
        a[k] = nmod_poly_get_coeff_ui(x, k);
}
static void set_coeffs(fq_nmod_t x, const uint64_t *a, const profile *f) {
    nmod_poly_zero(x);
    for (int k = 0; k < f->degree; k++)
        nmod_poly_set_coeff_ui(x, k, (ulong)a[k]);
}
static void assert_coeffs(const fq_nmod_t z, const uint64_t *want,
                          const profile *f) {
    if (nmod_poly_length(z) > f->degree) fail("unreduced result");
    for (int k = 0; k < f->degree; k++) {
        if (nmod_poly_get_coeff_ui(z, k) != want[k]) {
            fprintf(stderr, "%s coefficient %d\n", f->name, k);
            fail("FLINT differs from independent coefficient oracle");
        }
    }
    checks++;
}
static void perform_one(int op, profile *f, int i) {
    switch (op) {
    case ADD: fq_nmod_add(f->out+i, f->a+i, f->b+i, f->ctx); break;
    case SUB: fq_nmod_sub(f->out+i, f->a+i, f->b+i, f->ctx); break;
    case MUL: fq_nmod_mul(f->out+i, f->a+i, f->b+i, f->ctx); break;
    case SQUARE: fq_nmod_sqr(f->out+i, f->a+i, f->ctx); break;
    case SCALE: fq_nmod_mul_ui(f->out+i, f->a+i, f->scalars[i], f->ctx); break;
    case INVERSE: fq_nmod_inv(f->out+i, f->a+i, f->ctx); break;
    case FOLD:
        fq_nmod_sub(f->tmp1, f->b+i, f->a+i, f->ctx);
        fq_nmod_mul(f->tmp2, f->tmp1, f->challenge, f->ctx);
        fq_nmod_add(f->out+i, f->a+i, f->tmp2, f->ctx);
        break;
    default: fail("unknown operation");
    }
}
static void prepare_expected(profile *f, int i) {
    uint64_t a[MAX_DEGREE] = {0}, b[MAX_DEGREE] = {0};
    uint64_t r[MAX_DEGREE] = {0}, t[MAX_DEGREE] = {0};
    read_coeffs(a, f->a+i, f);
    read_coeffs(b, f->b+i, f);
    read_coeffs(r, f->challenge, f);
    for (int k = 0; k < f->degree; k++) {
        f->expected[ADD][i][k] = madd(a[k], b[k], f->p);
        f->expected[SUB][i][k] = msub(a[k], b[k], f->p);
        f->expected[SCALE][i][k] = mmul(a[k], f->scalars[i], f->p);
        t[k] = msub(b[k], a[k], f->p);
    }
    oracle_mul(f->expected[MUL][i], a, b, f);
    oracle_mul(f->expected[SQUARE][i], a, a, f);
    oracle_mul(f->expected[FOLD][i], t, r, f);
    for (int k = 0; k < f->degree; k++)
        f->expected[FOLD][i][k] = madd(a[k], f->expected[FOLD][i][k], f->p);
    /* Certify inverse by an independent multiplication before storing it. */
    if (!fq_nmod_is_zero(f->a+i, f->ctx)) {
        uint64_t inv[MAX_DEGREE] = {0}, product[MAX_DEGREE] = {0};
        perform_one(INVERSE, f, i);
        read_coeffs(inv, f->out+i, f);
        oracle_mul(product, a, inv, f);
        for (int k = 0; k < f->degree; k++)
            if (product[k] != (uint64_t)(k == 0)) fail("inverse oracle");
        memcpy(f->expected[INVERSE][i], inv, (size_t)f->degree * sizeof(uint64_t));
        checks++;
    }
}
static void verify_batch(profile *f, int op) {
    for (int i = 0; i < BATCH; i++) assert_coeffs(f->out+i, f->expected[op][i], f);
}
static void consume(profile *f) {
    uint64_t h = 0;
    for (int i = 0; i < BATCH; i++)
        for (int k = 0; k < f->degree; k++)
            h = (h ^ nmod_poly_get_coeff_ui(f->out+i, k)) * UINT64_C(0x100000001b3);
    sink ^= h;
}

/* Dispatch once, outside the timed inner loop; operands/output do not alias. */
#define RUN_LOOP(statement) \
    for (uint64_t pass = 0; pass < loops; pass++) \
        for (int i = 0; i < BATCH; i++) { statement; }
static void run_loops(profile *f, int op, uint64_t loops) {
    switch (op) {
    case ADD: RUN_LOOP(fq_nmod_add(f->out+i, f->a+i, f->b+i, f->ctx)); break;
    case SUB: RUN_LOOP(fq_nmod_sub(f->out+i, f->a+i, f->b+i, f->ctx)); break;
    case MUL: RUN_LOOP(fq_nmod_mul(f->out+i, f->a+i, f->b+i, f->ctx)); break;
    case SQUARE: RUN_LOOP(fq_nmod_sqr(f->out+i, f->a+i, f->ctx)); break;
    case SCALE: RUN_LOOP(fq_nmod_mul_ui(f->out+i, f->a+i, f->scalars[i], f->ctx)); break;
    case INVERSE: RUN_LOOP(fq_nmod_inv(f->out+i, f->a+i, f->ctx)); break;
    case FOLD:
        RUN_LOOP(
            fq_nmod_sub(f->tmp1, f->b+i, f->a+i, f->ctx);
            fq_nmod_mul(f->tmp2, f->tmp1, f->challenge, f->ctx);
            fq_nmod_add(f->out+i, f->a+i, f->tmp2, f->ctx)
        );
        break;
    default: fail("unknown benchmark operation");
    }
}
static void init_profile(profile *f) {
    nmod_poly_t modulus;
    double start = seconds(CLOCK_MONOTONIC);
    if (!n_is_prime((ulong)f->p)) fail("non-prime base modulus");
    nmod_poly_init(modulus, (ulong)f->p);
    nmod_poly_set_coeff_ui(modulus, 0, (ulong)(f->p - f->nonresidue));
    nmod_poly_set_coeff_ui(modulus, f->degree, 1);
    if (!nmod_poly_is_irreducible(modulus)) fail("reducible defining polynomial");
    fq_nmod_ctx_init_modulus(f->ctx, modulus, "X");
    nmod_poly_clear(modulus);
    f->context_seconds = seconds(CLOCK_MONOTONIC) - start;
    if (!f->ctx->sparse_modulus) fail("expected sparse reduction");
    fq_nmod_init(f->challenge, f->ctx);
    fq_nmod_init(f->tmp1, f->ctx);
    fq_nmod_init(f->tmp2, f->ctx);
    nmod_poly_fit_length(f->tmp1, 2*f->degree-1);
    nmod_poly_fit_length(f->tmp2, 2*f->degree-1);
    uint64_t seed = UINT64_C(0x464c494e5442454e) ^ f->p;
    uint64_t a[MAX_DEGREE], b[MAX_DEGREE], challenge[MAX_DEGREE];
    for (int k = 0; k < f->degree; k++) challenge[k] = random_mod(&seed, f->p);
    set_coeffs(f->challenge, challenge, f);
    for (int i = 0; i < BATCH; i++) {
        fq_nmod_init(f->a+i, f->ctx);
        fq_nmod_init(f->b+i, f->ctx);
        fq_nmod_init(f->out+i, f->ctx);
        nmod_poly_fit_length(f->out+i, 2*f->degree-1);
        for (int k = 0; k < f->degree; k++) {
            a[k] = random_mod(&seed, f->p);
            b[k] = random_mod(&seed, f->p);
        }
        set_coeffs(f->a+i, a, f);
        set_coeffs(f->b+i, b, f);
        if (fq_nmod_is_zero(f->a+i, f->ctx)) fq_nmod_one(f->a+i, f->ctx);
        f->scalars[i] = (ulong)random_mod(&seed, f->p);
        prepare_expected(f, i);
        for (int op = 0; op < NOPS; op++) {
            perform_one(op, f, i);
            assert_coeffs(f->out+i, f->expected[op][i], f);
        }
    }
    /* Edge vectors, including zero and maximal coefficients (especially u64). */
    uint64_t saved_a[MAX_DEGREE], saved_b[MAX_DEGREE];
    read_coeffs(saved_a, f->a, f);
    read_coeffs(saved_b, f->b, f);
    ulong saved_scalar = f->scalars[0];
    for (int edge = 0; edge < 6; edge++) {
        memset(a, 0, sizeof a);
        memset(b, 0, sizeof b);
        if (edge == 1) a[0] = b[0] = 1;
        if (edge == 2) a[0] = b[0] = f->p-1;
        if (edge == 3) a[f->degree-1] = b[f->degree-1] = f->p-1;
        if (edge == 4) for (int k=0; k<f->degree; k++) a[k] = b[k] = f->p-1;
        if (edge == 5) { a[1] = 1; b[f->degree-1] = 1; }
        set_coeffs(f->a, a, f);
        set_coeffs(f->b, b, f);
        f->scalars[0] = edge == 0 ? 0 : (ulong)(f->p-1);
        prepare_expected(f, 0);
        for (int op=0; op<NOPS; op++) {
            if (op == INVERSE && edge == 0) continue;
            perform_one(op, f, 0);
            assert_coeffs(f->out, f->expected[op][0], f);
        }
    }
    set_coeffs(f->a, saved_a, f);
    set_coeffs(f->b, saved_b, f);
    f->scalars[0] = saved_scalar;
    prepare_expected(f, 0);
}
static FILE *new_file(const char *directory, const char *name) {
    char path[4096];
    if (snprintf(path, sizeof path, "%s/%s", directory, name) >= (int)sizeof path)
        fail("output path too long");
    FILE *f = fopen(path, "wx");
    if (!f) { perror(path); fail("refusing to overwrite/missing output directory"); }
    return f;
}
int main(int argc, char **argv) {
    if (argc != 4) {
        fprintf(stderr, "usage: %s OUTPUT_DIRECTORY REPS TARGET_MILLISECONDS\n", argv[0]);
        return 2;
    }
    int reps = atoi(argv[2]);
    double target = atof(argv[3]) / 1000.0;
    if (reps < 1 || reps > 50 || !(target >= .01 && target <= 2)) fail("arguments");
    if (FLINT_BITS != 64 || sizeof(ulong) != 8) fail("requires 64-bit FLINT");
    flint_set_num_threads(1);
    FILE *csv = new_file(argv[1], "raw.csv");
    fprintf(csv, "field,p,degree,operation,rep,batch,loops,operations,wall_seconds,cpu_seconds,ns_per_op,cpu_ns_per_op,mops,checksum\n");
    for (int j=0; j<NFIELDS; j++) init_profile(profiles+j);
    FILE *meta = new_file(argv[1], "field_metadata.json");
    fprintf(meta, "{\n\"flint_version\":\"%s\",\"gmp_version\":\"%s\",\"mpfr_version\":\"%s\",\n", FLINT_VERSION, gmp_version, mpfr_get_version());
    fprintf(meta, "\"flint_bits\":%d,\"element_header_bytes\":%zu,\"threads\":%d,\"batch\":%d,\"reps\":%d,\"target_seconds\":%.6f,\n\"fields\":[\n", FLINT_BITS, sizeof(fq_nmod_struct), flint_get_num_threads(), BATCH, reps, target);
    for (int j=0; j<NFIELDS; j++) {
        profile *f = profiles+j;
        fprintf(meta, "{\"name\":\"%s\",\"p\":\"%" PRIu64 "\",\"degree\":%d,\"nonresidue\":%" PRIu64 ",\"polynomial\":\"X^%d-%" PRIu64 "\",\"log2_order\":%.9f,\"sparse_modulus\":%d,\"input_alloc_coefficients\":%ld,\"output_alloc_coefficients\":%ld,\"context_and_irreducibility_seconds\":%.9f}%s\n", f->name, f->p, f->degree, f->nonresidue, f->degree, f->nonresidue, f->degree*log2((double)f->p), f->ctx->sparse_modulus, (long)f->a[0].alloc, (long)f->out[0].alloc, f->context_seconds, j == NFIELDS-1 ? "" : ",");
    }
    fprintf(meta, "]}\n");
    fclose(meta);
    fprintf(stderr, "FLINT %s; independent arithmetic validation passed (%" PRIu64 " checks).\n", FLINT_VERSION, checks);
    /* Calibrate each cell outside measured trials, then fix its operation count. */
    for (int j=0; j<NFIELDS; j++) for (int op=0; op<NOPS; op++) {
        profile *f = profiles+j;
        uint64_t loops = 1;
        double elapsed;
        do {
            double start = seconds(CLOCK_MONOTONIC);
            run_loops(f, op, loops);
            elapsed = seconds(CLOCK_MONOTONIC) - start;
            consume(f);
            if (elapsed < .01) loops *= 2;
        } while (elapsed < .01);
        f->loops[op] = (uint64_t)ceil(loops * target / elapsed);
        if (!f->loops[op]) f->loops[op] = 1;
        verify_batch(f, op);
    }
    /* Rotate field and operation ordering to reduce order/thermal bias. */
    for (int rep=0; rep<reps; rep++) {
        for (int oi=0; oi<NOPS; oi++) for (int fi=0; fi<NFIELDS; fi++) {
            int op = (oi + rep) % NOPS;
            profile *f = profiles + (fi + rep) % NFIELDS;
            run_loops(f, op, 8);  /* untimed warm-up of this cell */
            double cpu_start = seconds(CLOCK_THREAD_CPUTIME_ID);
            double wall_start = seconds(CLOCK_MONOTONIC);
            run_loops(f, op, f->loops[op]);
            double wall = seconds(CLOCK_MONOTONIC) - wall_start;
            double cpu = seconds(CLOCK_THREAD_CPUTIME_ID) - cpu_start;
            consume(f);
            verify_batch(f, op);
            uint64_t operations = f->loops[op] * BATCH;
            fprintf(csv, "%s,%" PRIu64 ",%d,%s,%d,%d,%" PRIu64 ",%" PRIu64 ",%.9f,%.9f,%.6f,%.6f,%.6f,%" PRIu64 "\n", f->name, f->p, f->degree, op_names[op], rep+1, BATCH, f->loops[op], operations, wall, cpu, 1e9*wall/operations, 1e9*cpu/operations, operations/wall/1e6, (uint64_t)sink);
            fflush(csv);
        }
        fprintf(stderr, "Completed repetition %d/%d; all 28 result batches checked.\n", rep+1, reps);
    }
    fclose(csv);
    FILE *validation = new_file(argv[1], "c_validation.json");
    fprintf(validation, "{\"passed\":true,\"independent_oracle_checks\":%" PRIu64 ",\"flint_prime_and_irreducibility_checks\":4,\"edge_cases_per_field\":6,\"checksum\":%" PRIu64 "}\n", checks, (uint64_t)sink);
    fclose(validation);
    for (int j=0; j<NFIELDS; j++) {
        profile *f = profiles+j;
        for (int i=0; i<BATCH; i++) {
            fq_nmod_clear(f->a+i, f->ctx);
            fq_nmod_clear(f->b+i, f->ctx);
            fq_nmod_clear(f->out+i, f->ctx);
        }
        fq_nmod_clear(f->challenge, f->ctx);
        fq_nmod_clear(f->tmp1, f->ctx);
        fq_nmod_clear(f->tmp2, f->ctx);
        fq_nmod_ctx_clear(f->ctx);
    }
    flint_cleanup();
    return 0;
}
