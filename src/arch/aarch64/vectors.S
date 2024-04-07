.section EXCEPTIONS, "ax"
.global exceptions

.extern main

exceptions:
	mov sp, #0x100000
    b _start_rust 