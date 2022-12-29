package main

import (
	"fmt"
	wasmvm "github.com/zjg555543/movevm"
)

// This is just a demo to ensure we can compile a static go binary
func main() {
	version, _ := wasmvm.LibwasmvmVersion()
	fmt.Println("finished", version)

	wasmvm.LibPublish(123123123123)

	wasmvm.LibRun(1231)

	result, _ := wasmvm.LibInputOutput([]byte("12345678"))
	fmt.Println(result)

}
