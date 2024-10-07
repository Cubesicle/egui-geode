#ifndef FFI_H_
#define FFI_H_

void log_debug(char *str);
void log_info(char *str);
void log_warn(char *str);
void log_error(char *str);
const void *gl_get_proc_address(const char *str);

#endif // FFI_H_