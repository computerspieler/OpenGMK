
#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_render.h>
#include <SDL2/SDL_video.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_ttf.h>
#include <SDL2/SDL_mixer.h>

#include "room.hpp"

RoomEnvironment current_room;
int running;

int main(int argc, char* argv[])
{
    SDL_Event ev;
    SDL_Window* window = NULL;
    SDL_Renderer* renderer = NULL;

    if (SDL_Init(SDL_INIT_VIDEO) < 0) {
        fprintf(stderr, "SDL Initialization: %s", SDL_GetError());
        return -1;
    }

    window = SDL_CreateWindow(
        {{SCREEN_CAPTION}},
        {{SCREEN_POS_X}}, {{SCREEN_POS_Y}},
        {{SCREEN_WIDTH}}, {{SCREEN_HEIGHT}},
        SDL_WINDOW_SHOWN |
        {{SCREEN_RESIZABLE}} |
        {{SCREEN_BORDERLESS}} |
        0 //{{SCREEN_FULLSCREEN}}
    );
    if(!window) {
        fprintf(stderr, "Window initialization: %s", SDL_GetError());
        SDL_Quit();
        return - 1;
    }

    SDL_ShowCursor({{CURSOR_SHOW}});

    renderer = SDL_CreateRenderer(window, -1,
        SDL_RENDERER_ACCELERATED |
        SDL_RENDERER_TARGETTEXTURE
    );
    if(!renderer) {
        fprintf(stderr, "Window initialization: %s", SDL_GetError());
        SDL_Quit();
        return - 1;
    }
    
    SDL_RenderSetVSync(renderer, {{SCREEN_VSYNC}});

    running = 1;
    while(running) {
        SDL_SetRenderDrawColor(renderer,
            {{GAME_DEFAULT_BACKGROUND_COLOR_R}},
            {{GAME_DEFAULT_BACKGROUND_COLOR_G}},
            {{GAME_DEFAULT_BACKGROUND_COLOR_B}},
            255);
        SDL_RenderClear(renderer);

        current_room.update();
        current_room.draw();
        SDL_RenderPresent(renderer);
    }

    SDL_DestroyWindow(window);
    SDL_Quit();
    return 0;
}