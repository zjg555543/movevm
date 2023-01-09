package api

/*
#include "bindings.h"
#include <stdio.h>

// imports (db)
GoError cMoveSet(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, U8SliceView val, UnmanagedVector *errOut);
GoError cMoveGet(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut);
GoError cMoveDelete(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, UnmanagedVector *errOut);
GoError cMoveScan(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut);
// imports (iterator)
GoError cMoveNext(iterator_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, UnmanagedVector *key, UnmanagedVector *val, UnmanagedVector *errOut);
// imports (api)
GoError cMoveHumanAddress(api_t *ptr, U8SliceView src, UnmanagedVector *dest, UnmanagedVector *errOut, uint64_t *used_gas);
GoError cMoveCanonicalAddress(api_t *ptr, U8SliceView src, UnmanagedVector *dest, UnmanagedVector *errOut, uint64_t *used_gas);
// imports (querier)
GoError cMoveQueryExternal(querier_t *ptr, uint64_t gas_limit, uint64_t *used_gas, U8SliceView request, UnmanagedVector *result, UnmanagedVector *errOut);

// Gateway functions (db)
GoError cMoveGet_cgo(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut) {
	return cMoveGet(ptr, gas_meter, used_gas, key, val, errOut);
}
GoError cMoveSet_cgo(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, U8SliceView val, UnmanagedVector *errOut) {
	return cMoveSet(ptr, gas_meter, used_gas, key, val, errOut);
}
GoError cMoveDelete_cgo(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView key, UnmanagedVector *errOut) {
	return cMoveDelete(ptr, gas_meter, used_gas, key, errOut);
}
GoError cMoveScan_cgo(db_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut) {
	return cMoveScan(ptr, gas_meter, used_gas, start, end, order, out, errOut);
}

// Gateway functions (iterator)
GoError cMoveNext_cgo(iterator_t *ptr, gas_meter_t *gas_meter, uint64_t *used_gas, UnmanagedVector *key, UnmanagedVector *val, UnmanagedVector *errOut) {
	return cMoveNext(ptr, gas_meter, used_gas, key, val, errOut);
}

// Gateway functions (api)
GoError cMoveCanonicalAddress_cgo(api_t *ptr, U8SliceView src, UnmanagedVector *dest, UnmanagedVector *errOut, uint64_t *used_gas) {
    return cMoveCanonicalAddress(ptr, src, dest, errOut, used_gas);
}
GoError cMoveHumanAddress_cgo(api_t *ptr, U8SliceView src, UnmanagedVector *dest, UnmanagedVector *errOut, uint64_t *used_gas) {
    return cMoveHumanAddress(ptr, src, dest, errOut, used_gas);
}

// Gateway functions (querier)
GoError cMoveQueryExternal_cgo(querier_t *ptr, uint64_t gas_limit, uint64_t *used_gas, U8SliceView request, UnmanagedVector *result, UnmanagedVector *errOut) {
    return cMoveQueryExternal(ptr, gas_limit, used_gas, request, result, errOut);
}
*/
import "C"

// We need these gateway functions to allow calling back to a go function from the c code.
// At least I didn't discover a cleaner way.
// Also, this needs to be in a different file than `callbacks.go`, as we cannot create functions
// in the same file that has //export directives. Only import header types
