package main

/*
#cgo LDFLAGS: -L../../target/release -laslang
#include <stdlib.h>

extern char* as_execute(const char* code);
extern void as_free_string(char* s);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// Execute runs AS Lang code and returns the output string
func Execute(code string) string {
	cCode := C.CString(code)
	defer C.free(unsafe.Pointer(cCode))

	cResult := C.as_execute(cCode)
	defer C.as_free_string(cResult)

	return C.GoString(cResult)
}

func main() {
	code := `print("Hello from Go!"); let x = 40 + 2; print(x);`
	fmt.Printf("Running AS Lang from Go...\n")
	output := Execute(code)
	fmt.Printf("Output:\n%s\n", output)
}
