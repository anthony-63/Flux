#include "game.h"

#include <stdlib.h>

flux_game* flux_game_init(flux_logger* logger) {
    flux_game* game = malloc(sizeof * game);
    
    game->logger = logger;

    game->window = SDL_CreateWindow(WINDOW_TITLE, SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, WINDOW_WIDTH, WINDOW_HEIGHT, 0);
    if(!game->window) {
        flux_err(game->logger, "Failed to open a window.\nErr: %s\n", SDL_GetError());
    }
    
    flux_info(game->logger, "Created a window: 0x%X\n", &game->window);

    SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");

    game->renderer = SDL_CreateRenderer(game->window, -1, SDL_RENDERER_ACCELERATED);
    if(!game->renderer) {
        flux_err(game->logger, "Failed to create renderer for window.\nErr: %s\n", SDL_GetError());
    }
    flux_info(game->logger, "Created a renderer: 0x%X\n", &game->renderer);

    game->border = flux_border_init(game->logger, game->renderer);

    return game;
}

void handle_events(flux_game* game) {
    SDL_Event ev;
    while(SDL_PollEvent(&ev)) {
        switch(ev.type) {
            case SDL_QUIT:
                exit(0);
                break;
            default:
                break;
        }
    }
}

void update(flux_game* game) {

}

void prepare_scene(flux_game* game) {
    SDL_SetRenderDrawColor(game->renderer, 0, 0, 0, 0);
    SDL_RenderClear(game->renderer);
    flux_border_draw(game->border, game->renderer);
}

void present_scene(flux_game* game) {
    SDL_RenderPresent(game->renderer);
}

void flux_game_play_map(flux_game* game, flux_map map) {
    for(;;) {
        prepare_scene(game);
        handle_events(game);
        present_scene(game);
    }
}