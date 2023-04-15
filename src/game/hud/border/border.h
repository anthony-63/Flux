#pragma once

#include <SDL2/SDL.h>

#define BORDER_IMAGE_PATH "data/border.png"
#define BORDER_WINDOW_DIVIDER 2.7

#include "../../../logger/logger.h"


typedef struct {
    SDL_Texture* texture;
    SDL_Rect rect;
} flux_border;

flux_border* flux_border_init(flux_logger* logger, SDL_Renderer* renderer);
void flux_border_draw(flux_border* border, SDL_Renderer* renderer);