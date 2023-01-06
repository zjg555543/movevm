package main

import (
	"fmt"
	wasmvm "github.com/zjg555543/movevm"
	"github.com/zjg555543/movevm/api"
	"github.com/zjg555543/movevm/types"
	"io/ioutil"
)

// This is just a demo to ensure we can compile a static go binary
func main() {
	version, _ := wasmvm.Version()
	fmt.Println("finished", version)

	gasMeter := api.NewMockGasMeter(api.TESTING_GAS_LIMIT)

	store := api.NewLookup(gasMeter)

	testByte := []byte("1234567890")

	moduleBytes := readModule("/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/Test.mv")

	sender2 := []byte("0x2")

	pathList := [...]string{
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/debug.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/fixed_point32.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/hash.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/vector.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/errors.mv", "/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/acl.mv", "/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/option.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/ascii.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/bit_vector.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/signer.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/error.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/capability.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/compare.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/guid.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/bcs.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/event.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/offer.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/role.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/string.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveStdlib/type_name.mv",
		"/Users/oker/workspace/move/movevm/contracts/readme/build/readme/bytecode_modules/dependencies/MoveNursery/vault.mv",
	}
	for _, s := range pathList {
		sender1 := []byte("0x1")
		wasmvm.Publish(readModule(s), sender1, testByte, gasMeter, store, nil, nil, 10000, false)
	}

	wasmvm.Publish(moduleBytes, sender2, testByte, gasMeter, store, nil, nil, 10000, false)

	wasmvm.Run(testByte, testByte, testByte, gasMeter, store, nil, nil, 10000, false)

	balance := types.Coins{types.NewCoin(250, "ATOM")}
	querier := api.DefaultQuerier(api.MOCK_CONTRACT_ADDR, balance)
	result, err := wasmvm.InputOutput(testByte, testByte, testByte, gasMeter, store, nil, &querier, 10000, false)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(result)
	}
}

func readModule(path string) []byte {
	f, err := ioutil.ReadFile(path)
	if err != nil {
		fmt.Println("read fail", err)
		return nil
	}

	return f
}
