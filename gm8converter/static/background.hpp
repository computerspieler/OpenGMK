#ifndef _BACKGROUND_HPP_
#define _BACKGROUND_HPP_

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_video.h>

#include "utils.hpp"

struct TileFormat {
    const Vector2 offset;
    const Vector2 separation;
    const Vector2 size;
};

class Background {
public:
    Background(
        std::string path,
        bool smooth_edges,
        bool transparent,
        bool preload,
        TileFormat tiles
    ):
        m_surface(NULL),
        m_path(path),
        m_tiles(tiles),
        m_use_as_tileset(true),
        m_smooth_edges(smooth_edges),
        m_transparent(transparent)
    {
        if(preload)
            load();
    }

    Background(
        std::string path,
        bool smooth_edges,
        bool transparent,
        bool preload
    ):
        m_surface(NULL),
        m_path(path),
        m_tiles((TileFormat) {}),
        m_use_as_tileset(false),
        m_smooth_edges(smooth_edges),
        m_transparent(transparent)
    {
        if(preload)
            load();
    }

    virtual void load() { _load(); }
    virtual void unload() { _unload(); }
    
    ~Background() {
        _unload();
    }

protected:
    void _load() {
        m_surface = SDL_LoadBMP(m_path.c_str());
    }
   
    void _unload() {
        SDL_FreeSurface(m_surface);
    }

private:
    SDL_Surface *m_surface;

    std::string m_path;
    TileFormat m_tiles;
    bool m_use_as_tileset;
    bool m_smooth_edges;
    bool m_transparent;
};

extern const Background backgrounds[];

#endif /* _BACKGROUND_HPP_ */
