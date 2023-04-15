#pragma once

#include <stdio.h>

typedef struct {
    FILE* log_file;
} flux_logger;

flux_logger* flux_logger_init(const char* file_path);
void flux_log_none(flux_logger* logger, char* fmt, ...);
void flux_info(flux_logger* logger, char* fmt, ...);
void flux_warn(flux_logger* logger, char* fmt, ...);
void flux_err(flux_logger* logger, char* fmt, ...);