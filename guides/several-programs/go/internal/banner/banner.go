package banner

import "fmt"

// Shared code for every program in this module.
func For(program string) string { return fmt.Sprintf("== %s ==", program) }
