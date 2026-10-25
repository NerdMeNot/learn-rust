package main

/*
#cgo CFLAGS: -I${SRCDIR}/..
#cgo LDFLAGS: ${SRCDIR}/../target/debug/libbridge.a -lpthread -ldl -lm
#include <stdlib.h>
#include "bridge.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	a := C.Point{x: 0, y: 0}
	b := C.Point{x: 3, y: 4}
	fmt.Println("distance:", float64(C.point_distance(a, b)))

	numbers := []C.int32_t{10, 20, 30}
	fmt.Println("sum:", int64(C.sum_i32(&numbers[0], C.uintptr_t(len(numbers)))))

	name := C.CString("Go") // allocated by C's malloc: Go's garbage collector does not manage it
	defer C.free(unsafe.Pointer(name))
	text := C.greeting(name)
	fmt.Println(C.GoString(text)) // copies the bytes into a Go string
	C.greeting_free(text)
}
