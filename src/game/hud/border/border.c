#include "border.h"
#include "../../game.h"

#include <stdlib.h>
#include <SDL2/SDL_image.h>

flux_border* flux_border_init(flux_logger* logger, SDL_Renderer* renderer) {
    flux_border* border = malloc(sizeof * border);

    border->rect.w = WINDOW_WIDTH / BORDER_WINDOW_DIVIDER;
    border->rect.h = WINDOW_WIDTH / BORDER_WINDOW_DIVIDER;

    border->rect.x = (WINDOW_WIDTH / 2) - border->rect.w / 2;
    border->rect.y = (WINDOW_HEIGHT / 2) - border->rect.h / 2;

    SDL_Surface* surface = IMG_Load(BORDER_IMAGE_PATH);
    if(!surface) {
        flux_err(logger, "Failed to load border image %s\nErr: %s\n", BORDER_IMAGE_PATH, IMG_GetError());
    }

    border->texture = SDL_CreateTextureFromSurface(renderer, surface);
    if(!border->texture) {
        flux_err(logger, "Failed to create border texture from surface %s\nErr: %s\n", BORDER_IMAGE_PATH, IMG_GetError());
    }

    SDL_FreeSurface(surface);

    return border;
}

void flux_border_draw(flux_border* border, SDL_Renderer* renderer) {
    SDL_RenderCopy(renderer, border->texture, NULL, &border->rect);
}
