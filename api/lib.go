package api

/*
#include "bindings.h"
*/
import "C"
import (
	"fmt"
	"github.com/zjg555543/movevm/types"
	"runtime"
	"syscall"
)

// Value types
type (
	cint   = C.int
	cbool  = C.bool
	cusize = C.size_t
	cu8    = C.uint8_t
	cu32   = C.uint32_t
	cu64   = C.uint64_t
	ci8    = C.int8_t
	ci32   = C.int32_t
	ci64   = C.int64_t
)

// Pointers
type cu8_ptr = *C.uint8_t

type Querier = types.Querier

func ApiMoveVersion() (string, error) {
	version_ptr, err := C.version_str()
	if err != nil {
		return "", err
	}
	// For C.GoString documentation see https://pkg.go.dev/cmd/cgo and
	// https://gist.github.com/helinwang/2c7bd2867ea5110f70e6431a7c80cd9b
	version_copy := C.GoString(version_ptr)
	return version_copy, nil
}

func ApiPublish(gas_limited uint64) {
	C.say_publish(cu64(gas_limited))
}

func ApiRun(gas_limited uint64) {
	C.say_run(cu64(gas_limited))
}

func ApiInputOutput(input []byte) ([]byte, error) {
	//C.say_input_output(cu64(gas_limited))

	w := makeView(input)
	defer runtime.KeepAlive(input)
	errmsg := newUnmanagedVector(nil)

	db := buildDB(nil, nil)
	a := buildAPI(nil)
	q := buildQuerier(nil)

	result, err := C.say_input_output(w, db, a, q, &errmsg)
	if err != nil {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(result), nil

}

func errorWithMessage(err error, b C.UnmanagedVector) error {
	// this checks for out of gas as a special case
	if errno, ok := err.(syscall.Errno); ok && int(errno) == 2 {
		return types.OutOfGasError{}
	}
	msg := copyAndDestroyUnmanagedVector(b)
	if msg == nil {
		return err
	}
	return fmt.Errorf("%s", string(msg))
}
