#pragma once

#define WINDOW_WIDTH 1280
#define WINDOW_HEIGHT 720

#define VERSION_STR "v0.1 ALPHA"
#define WINDOW_TITLE "Flux(Proving A Point) | " VERSION_STR

#include <SDL2/SDL.h>

#include "../logger/logger.h"
#include "../loaders/map.h"
#include "hud/border/border.h"

typedef struct {
    SDL_Renderer* renderer;
    SDL_Window* window;
    flux_logger* logger;
    flux_border* border;
} flux_game;

flux_game* flux_game_init(flux_logger* logger);
void flux_game_play_map(flux_game* game, flux_map map);
