package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: ./lib/libhello.a -ldl
#include "./lib/hello.h"
*/
import "C"
import (
	"fmt"
	"unsafe"

	"go.opentelemetry.io/collector/pdata/ptrace"
)

func main() {
	C.hello(C.CString("world"))
	C.whisper(C.CString("this is code from the static library"))
	// C.run_tokio()
	// time.Sleep(10 * time.Second)
	fmt.Println("now time for some tracing")

	tr := ptrace.NewTraces()
	tr.ResourceSpans().AppendEmpty().Resource().Attributes().PutStr("blep", "nghuu")

	uns := unsafe.Pointer(&tr)
	fmt.Printf("address in Go: %d \n", uns)

	C.test_pointer((*C.char)(uns))

	fmt.Println("dziaa na")
}
