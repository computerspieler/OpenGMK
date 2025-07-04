#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_video.h>

#include "utils.hpp"

struct KeyFrame {
    SDL_Surface *surface;
    std::string path;
};

extern const KeyFrame keyframes[];

class Sprite;

class SpriteInstance {
public:
    SpriteInstance(const Sprite *src) :
        m_src(src),
        current_keyframe(0)
    {}

    void update();
    void draw();

private:
    const Sprite *m_src;
    int current_keyframe;
};

class Sprite {
public:
    Sprite(
        Vector2 origin,
        bool preload,
        bool smooth_edges,
        bool transparent,
        int first_keyframe,
        int keyframe_count
    ) :
        m_origin(origin),
        m_preload(preload),
        m_smooth_edges(smooth_edges),
        m_transparent(transparent),
        m_first_keyframe(first_keyframe),
        m_keyframe_count(keyframe_count)
    {}

    int keyframe_count() const
    { return m_keyframe_count; }

    SpriteInstance build_instance() const {
        SpriteInstance out(this);
        return out;
    }

private:
    Vector2 m_origin;
    bool m_preload;
    bool m_smooth_edges;
    bool m_transparent;

    int m_first_keyframe;
    int m_keyframe_count;

    //TODO: SpriteMask
};

extern const Sprite sprites[];
