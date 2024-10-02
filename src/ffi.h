#ifndef FFI_H_
#define FFI_H_

void _log_debug(char *str);
void _log_info(char *str);
void _log_warn(char *str);
void _log_error(char *str);
const void * _gl_get_proc_address(const char *str);

#endif // FFI_H_