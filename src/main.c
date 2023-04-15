#include <stdio.h>

#define SDL_MAIN_HANDLED
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

#include "logger/logger.h"
#include "loaders/map.h"
#include "game/game.h"

#define FLUX_LOG_FILE "data/flux.log.txt"


int main(int argc, char** argv) {
    flux_logger* logger = flux_logger_init(FLUX_LOG_FILE);

    if(SDL_Init(SDL_INIT_AUDIO | SDL_INIT_VIDEO)) {
        flux_err(logger, "Failed to init SDL2\nErr: %s\n", SDL_GetError());
    }

    int img_flags = IMG_INIT_PNG | IMG_INIT_JPG;

    if(!(IMG_Init(img_flags) & img_flags)) {
        flux_err(logger, "Failed to initialize SDL_Image.\nErr: %s\n", IMG_GetError());
    }

    flux_map map = load_map(logger, "data/maps/ss_archive_a-ha_-_take_on_me__milkshake_s_silly_dnb_mix_.flux");

    flux_game* game = flux_game_init(logger);

    flux_game_play_map(game, map);

    return 0;
}