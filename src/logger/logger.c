#include "logger.h"

#include <stdlib.h>
#include <stdarg.h>
#include <locale.h>

flux_logger* flux_logger_init(const char* file_path) {
    flux_logger* logger = malloc(sizeof * logger);
    
    logger->log_file = fopen(file_path, "w+");

    flux_info(logger, "Start of log.\n");

    return logger;
}

void flux_log(flux_logger* logger, char* symbol, char* fmt, va_list args) {
    // write log to stdout
    printf("[%s FLUX] ", symbol);
    vprintf(fmt, args);

    // write log to file
    fprintf(logger->log_file, "[%s FLUX] ", symbol);
    vfprintf(logger->log_file, fmt, args);
}

void flux_log_no_logo(flux_logger* logger, char* fmt, va_list args) {
    // write log to stdout
    vprintf(fmt, args);

    // write log to file
    vfprintf(logger->log_file, fmt, args);
}

void flux_info(flux_logger* logger, char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    flux_log(logger, ":)", fmt, args); // info symbol

    va_end(args);
}

void flux_log_none(flux_logger* logger, char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    flux_log_no_logo(logger, fmt, args); // info symbol

    va_end(args);
}

void flux_warn(flux_logger* logger, char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    flux_log(logger, ":o", fmt, args);

    va_end(args);
}

void flux_err(flux_logger* logger, char* fmt, ...) {
    va_list args;
    va_start(args, fmt);

    flux_log(logger, ":(", fmt, args);

    va_end(args);

    exit(-1);
}
