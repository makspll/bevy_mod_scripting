/* Reads and writes Rust values through the generated overlays, like script FFI code would. */
#include "overlays.h"

#include <stdlib.h>

#ifdef _MSC_VER
#include <malloc.h>
#define bms_aligned_alloc(align, size) _aligned_malloc((size), (align))
#define bms_aligned_free(ptr) _aligned_free(ptr)
#else
#define bms_aligned_alloc(align, size) aligned_alloc((align), (size))
#define bms_aligned_free(ptr) free(ptr)
#endif

static float *vec3_field(Vec3C *v, int i) { return i == 0 ? &v->x : i == 1 ? &v->y : &v->z; }
static float *vec3a_field(Vec3AC *v, int i) { return i == 0 ? &v->x : i == 1 ? &v->y : &v->z; }
static Vec3C *mat3_col(Mat3C *m, int col) { return col == 0 ? &m->x_axis : col == 1 ? &m->y_axis : &m->z_axis; }

float bms_ffi_vec3_get(const Vec3C *v, int i) { return *vec3_field((Vec3C *)v, i); }
void bms_ffi_vec3_set(Vec3C *v, int i, float value) { *vec3_field(v, i) = value; }

float bms_ffi_vec3a_get(const Vec3AC *v, int i) { return *vec3a_field((Vec3AC *)v, i); }
void bms_ffi_vec3a_set(Vec3AC *v, int i, float value) { *vec3a_field(v, i) = value; }

float bms_ffi_mat3_get(const Mat3C *m, int col, int row) { return *vec3_field(mat3_col((Mat3C *)m, col), row); }
void bms_ffi_mat3_set(Mat3C *m, int col, int row, float value) { *vec3_field(mat3_col(m, col), row) = value; }

/* A value created on the C side (like LuaJIT's `ffi.new`), handed to Rust as the real type. */
Vec3AC *bms_ffi_vec3a_new(float x, float y, float z) {
    Vec3AC *v = bms_aligned_alloc(_Alignof(Vec3AC), sizeof(Vec3AC));
    if (v) {
        v->x = x;
        v->y = y;
        v->z = z;
        v->_w = z;
    }
    return v;
}

void bms_ffi_vec3a_free(Vec3AC *v) { bms_aligned_free(v); }
