package main

import (
	"fmt"
	wasmvm "github.com/zjg555543/movevm"
	"github.com/zjg555543/movevm/api"
	"github.com/zjg555543/movevm/types"
)

// This is just a demo to ensure we can compile a static go binary
func main() {
	version, _ := wasmvm.Version()
	fmt.Println("finished", version)

	gasMeter := api.NewMockGasMeter(api.TESTING_GAS_LIMIT)

	store := api.NewLookup(gasMeter)

	testByte := []byte("12345678")
	wasmvm.Publish(testByte, testByte, testByte, gasMeter, store, nil, nil, 10000, false)

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
