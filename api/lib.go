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
	version_copy := C.GoString(version_ptr)
	return version_copy, nil
}

func ApiBuild(gasMeter *GasMeter, store KVStore) {
	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState, gasMeter)
	C.say_build(db)
}

func ApiPublish(env []byte, info []byte, msg []byte, gasMeter *GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {

	w := makeView(env)
	defer runtime.KeepAlive(env)

	i := makeView(info)
	defer runtime.KeepAlive(info)

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState, gasMeter)
	C.say_publish(w, i, db)
}

func ApiRun(codeByte []byte, info []byte, msg []byte, gasMeter *GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) {

	cCodeByte := makeView(codeByte)
	defer runtime.KeepAlive(codeByte)

	cInfo := makeView(info)
	defer runtime.KeepAlive(info)

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	cDb := buildDB(&dbState, gasMeter)
	cApi := buildAPI(goApi)
	cQuerier := buildQuerier(querier)

	C.say_run(cCodeByte, cInfo, cDb, cApi, cQuerier)
}

const TESTING_GAS_LIMIT = uint64(500_000_000_000) // ~0.5ms
func ApiInputOutput(env []byte, info []byte, msg []byte, gasMeter *GasMeter, store KVStore,
	goApi *GoAPI, querier *Querier, gasLimit uint64, printDebug bool) ([]byte, error) {

	w := makeView(env)
	defer runtime.KeepAlive(env)
	errmsg := newUnmanagedVector(nil)

	//api := NewMockAPI()

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	cDb := buildDB(&dbState, gasMeter)
	cApi := buildAPI(goApi)
	cQuerier := buildQuerier(querier)

	result, err := C.say_input_output(w, cDb, cApi, cQuerier, &errmsg)
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
