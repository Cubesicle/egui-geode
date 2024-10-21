#pragma once

void log_debug(char *str);
void log_info(char *str);
void log_warn(char *str);
void log_error(char *str);
const uintptr_t gl_get_proc_address(const char *str);