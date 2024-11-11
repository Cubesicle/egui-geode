#pragma once

void log_debug(const char *str);
void log_info(const char *str);
void log_warn(const char *str);
void log_error(const char *str);
const void *gl_get_proc_address(const char *str);