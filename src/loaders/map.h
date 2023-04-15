#pragma once

#include <stdint.h>
#include "../logger/logger.h"

#define MAX_MAP_NOTES 0xFFFFFF

typedef struct  {
    uint32_t time;
    float x, y;
} flux_map_note;

typedef struct {
    char* name;
    size_t note_len;
    flux_map_note notes[MAX_MAP_NOTES];
} flux_difficulty;

typedef struct {
    uint8_t* music_data;
    uint8_t* image_data;
    char* artist;
    char* mapper;
    char* song_name;
    int has_artist;
    int has_image;
    flux_difficulty** difficulties;
} flux_map;

typedef struct {
    flux_map* maps;
    int map_count;
} flux_maploader;

flux_maploader* flux_maploader_init(flux_logger* logger, char* map_folder);
flux_map load_map(flux_logger* logger, char* map_path);
