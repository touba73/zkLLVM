#include <cstdint>

[[circuit]] uint64_t shl(uint64_t value, uint64_t shift, uint64_t expected_shifted)
{
    uint64_t shifted = value << shift;
    __builtin_assigner_exit_check(shifted == expected_shifted);
    return shifted;
}
