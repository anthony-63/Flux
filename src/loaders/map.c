#include "map.h"

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

uint64_t read_u64(uint8_t* data, size_t* index) {
    uint64_t bytes[8];
    bytes[0] = data[(*index)++];
    bytes[1] = data[(*index)++];
    bytes[2] = data[(*index)++];
    bytes[3] = data[(*index)++];
    bytes[4] = data[(*index)++];
    bytes[5] = data[(*index)++];
    bytes[6] = data[(*index)++];
    bytes[7] = data[(*index)++];
    return (bytes[0] << 56) | (bytes[1] << 48) | (bytes[2] << 40) | (bytes[3] << 32) | (bytes[4] << 24) | (bytes[5] << 16) | (bytes[6] << 8) | bytes[7] ;
}

uint32_t read_u32(uint8_t* data, size_t* index) {
    uint32_t bytes[4];
    bytes[0] = data[(*index)++];
    bytes[1] = data[(*index)++];
    bytes[2] = data[(*index)++];
    bytes[3] = data[(*index)++];
    return (bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3] ;
}

uint16_t read_u16(uint8_t* data, size_t* index) {
    uint16_t bytes[2];
    bytes[0] = data[(*index)++];
    bytes[1] = data[(*index)++];
    return (bytes[0] << 8) | bytes[1];
}

uint8_t read_u8(uint8_t* data, size_t* index) {
    return data[(*index)++];
}

float read_float(uint8_t* data, size_t* index) {
    uint32_t b = read_u32(data, index);
    float f;
    memcpy(&f, &b, sizeof(f));
    return f;
}

flux_map load_map(flux_logger* logger, char* map_path) {
    FILE* file = fopen(map_path, "r");
    if(!file) flux_err(logger, "Failed to open map file '%s'\n", map_path);

    uint8_t* map_data;

    fseek(file, 0L, SEEK_END);
    long file_size = ftell(file);
    rewind(file);

    map_data = malloc(file_size + 1);
    if(!map_data) fclose(file), flux_err(logger, "Failed to create map buffer\n");

    fread(map_data, file_size, 1, file);

    size_t map_index = 0;
    char magic[5];
    uint8_t version = 0;

    magic[0] = read_u8(map_data, &map_index);
    magic[1] = read_u8(map_data, &map_index);
    magic[2] = read_u8(map_data, &map_index);
    magic[3] = read_u8(map_data, &map_index);
    magic[4] = '\0';

    if(magic[0] != 'F' || magic[1] != 'L' || magic[2] != 'U' || magic[3] != 'X'){
        flux_err(logger, "Invalid bytes(%c%c%c%c) at the start of map file '%s'\n", magic[0], magic[1], magic[2], magic[3], map_path);
    }

    version = read_u8(map_data, &map_index);

    flux_info(logger, "Flux map version: %d\n", version);

    if(version != 1) { //current flux map version 
        flux_err(logger, "Invalid map version '%d' for map '%s'\n", version, map_path);
    }

    size_t meta_count = read_u16(map_data, &map_index);
    char* mapper;
    char* artist;
    char* song_name;
    int has_artist = 0;


    // load map metadata
    for(int i = 0; i < meta_count; i++) {
        size_t metadata_key_length = read_u16(map_data, &map_index);
        
        char metadata_key[metadata_key_length + 1];
        for(int j = 0; j < metadata_key_length; j++) {
            metadata_key[j] = read_u8(map_data, &map_index);
        }
        metadata_key[metadata_key_length] = '\0';  

        size_t metadata_val_length = read_u32(map_data, &map_index);

        char metadata_val[metadata_val_length + 1];
        for(int j = 0; j < metadata_val_length; j++) {
            metadata_val[j] = read_u8(map_data, &map_index);
        }
        metadata_val[metadata_val_length] = '\0';

        if(!strcmp("artist", metadata_key)) {
            has_artist = 1;
            artist = malloc((metadata_val_length + 1) * sizeof(char));
            strcpy(artist, metadata_val);
            artist[strcspn(artist, "\r\n")] = 0;
        }else if(!strcmp("song_name", metadata_key)) {
            song_name = malloc((metadata_val_length + 1) * sizeof(char));
            strcpy(song_name, metadata_val);
            song_name[strcspn(song_name, "\r\n")] = 0;
        }else if(!strcmp("mapper", metadata_key)) {
            mapper = malloc((metadata_val_length + 1) * sizeof(char));
            strcpy(mapper, metadata_val);
            mapper[strcspn(mapper, "\r\n")] = 0;
        }
    }

    // load all map data and diffs

    size_t diff_count = read_u16(map_data, &map_index);
    flux_difficulty** difficulties = malloc(sizeof(flux_difficulty) * diff_count);
    for(int i = 0; i < diff_count; i++) {
        size_t diff_name_len = read_u16(map_data, &map_index);

        char diff_name[diff_name_len + 1];
        for(int j = 0; j < diff_name_len; j++) {
            diff_name[j] = read_u8(map_data, &map_index);
        }
        diff_name[diff_name_len] = '\0';

        difficulties[i] = malloc(sizeof(flux_difficulty));
        difficulties[i]->name = malloc((diff_name_len + 1) * sizeof(char));
        strcpy(difficulties[i]->name, diff_name);
        difficulties[i]->name[strcspn(difficulties[i]->name, "\r\n")] = 0;

        size_t note_count = read_u64(map_data, &map_index);
        for(int j = 0; j < note_count; j++) {
            uint32_t time = read_u32(map_data, &map_index);
            float x = read_float(map_data, &map_index);
            float y = read_float(map_data, &map_index);
            difficulties[i]->notes[i] = (flux_map_note) {
                .x = x,
                .y = y,
                .time = time,
            };
        }
    }
    
    // load map jacket image

    int has_image;
    size_t image_size = read_u32(map_data, &map_index);
    has_image = image_size != 0;
    uint8_t* image = malloc(image_size);
    if(has_image) {
        for(int i = 0; i < image_size; i++) {
            image[i] = read_u8(map_data, &map_index);
        }
    }

    // load map audio
    size_t music_size = read_u32(map_data, &map_index);
    uint8_t* music = malloc(music_size);
    for(int i = 0; i < music_size; i++) {
        music[i] = read_u8(map_data, &map_index);
    }
    
    flux_info(logger, "Loaded map: {mapper: %s, has_artist: %d, artist: %s, song_name: %s, difficulties: {", mapper, has_artist, artist, song_name);

    for(int i = 0; i < diff_count; i++) {
        if(i == diff_count - 1) flux_log_none(logger, "%s}}\n", difficulties[i]->name);
        else flux_log_none(logger, "%s,", difficulties[i]->name);
    }

    fclose(file);
    free(map_data);

    return (flux_map) {
        .artist = artist,
        .has_artist = has_artist,
        .mapper = mapper,
        .difficulties = difficulties,
        .has_image = has_image,
        .image_data = image,
        .music_data = music,
        .song_name = song_name,
    };

}

flux_maploader* flux_maploader_init(flux_logger* logger, char* map_folder) {

}
