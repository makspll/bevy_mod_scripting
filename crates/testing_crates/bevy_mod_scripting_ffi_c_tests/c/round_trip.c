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

/* One representative per glam family, elements by flat index (the order of glam's `to_cols_array`). */
#define BMS_ACCESS(T, E, N, ...)                                                                                \
    static E *T##_element(T *v, int i) {                                                                        \
        E *elements[N] = {__VA_ARGS__};                                                                         \
        return elements[i];                                                                                     \
    }                                                                                                           \
    E bms_ffi_##T##_get(const T *v, int i) { return *T##_element((T *)v, i); }                                  \
    void bms_ffi_##T##_set(T *v, int i, E value) { *T##_element(v, i) = value; }

BMS_ACCESS(Vec4C, float, 4, &v->x, &v->y, &v->z, &v->w)
BMS_ACCESS(QuatC, float, 4, &v->x, &v->y, &v->z, &v->w)
BMS_ACCESS(Mat4C, float, 16, &v->x_axis.x, &v->x_axis.y, &v->x_axis.z, &v->x_axis.w, &v->y_axis.x, &v->y_axis.y,
           &v->y_axis.z, &v->y_axis.w, &v->z_axis.x, &v->z_axis.y, &v->z_axis.z, &v->z_axis.w, &v->w_axis.x,
           &v->w_axis.y, &v->w_axis.z, &v->w_axis.w)
BMS_ACCESS(Affine2C, float, 6, &v->matrix2.x_axis.x, &v->matrix2.x_axis.y, &v->matrix2.y_axis.x,
           &v->matrix2.y_axis.y, &v->translation.x, &v->translation.y)
BMS_ACCESS(DVec3C, double, 3, &v->x, &v->y, &v->z)
BMS_ACCESS(IVec3C, int32_t, 3, &v->x, &v->y, &v->z)
BMS_ACCESS(U8Vec4C, uint8_t, 4, &v->x, &v->y, &v->z, &v->w)
BMS_ACCESS(USizeVec2C, uintptr_t, 2, &v->x, &v->y)
BMS_ACCESS(BVec3C, bool, 3, &v->x, &v->y, &v->z)
