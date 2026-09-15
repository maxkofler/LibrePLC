#include "libreplc.h"
#include <stddef.h>

uint32_t __lplc_setup(size_t context, size_t (*syscall)(size_t, RtCall, void*)) {
    int var = 0;

    syscall(context, RT_CALL_NOP, &var);

    return 0;
}
