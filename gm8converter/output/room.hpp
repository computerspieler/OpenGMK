#ifndef _ROOM_HPP_
#define _ROOM_HPP_

#include "utils.hpp"
#include "object.hpp"

#include <SDL2/SDL_events.h>
#include <cstddef>
#include <vector>

class RoomEnvironment;

class Room {
public:
    Room(
        int     speed,
        bool    persistent,
        bool    draw_background_color,
        bool    enable_views,
        Vector2 size,
        bool    isometric,
        Vector2 snap,
        Color   background_color
    ) :
        m_speed(speed),
        m_persistent(persistent),
        m_draw_background_color(draw_background_color),
        m_enable_views(enable_views),
        m_size(size),
        m_isometric(isometric),
        m_snap(snap),
        m_background_color(background_color)
    {}

    void setup(RoomEnvironment *env);

private:
    int     m_speed;
    bool    m_persistent;
    bool    m_draw_background_color;
    bool    m_enable_views;
    Vector2 m_size;
    bool    m_isometric;
    Vector2 m_snap;
    Color   m_background_color;
};

class RoomEnvironment {
public:
    RoomEnvironment() :
        curent_room(NULL),
        m_instances()
    {}

    void set_current_room(Room *room) {
        //this->tiles.clear();
        this->m_instances.clear();
        this->curent_room = room;
        if(room)
            room->setup(this);
    }

    void add_instance(ObjectInstance &i) {
        this->m_instances.push_back(i);
    }

    void update() {
        SDL_Event ev;
        if(!curent_room)
            return;
    
        //Begin step events
        for(ObjectInstance &i: this->m_instances)
            i.m_generator->on_begin_step(&i);
                    
        //Alarm events
        for(ObjectInstance &i: this->m_instances)
            for(int idx = 0; idx < 12; idx ++) {
                i.m_alarms[idx].update();
                if(i.m_alarms[idx].fire_event())
                    i.m_generator->on_alarm(&i, idx);
            }

        while(SDL_PollEvent(&ev)) {
            switch (ev.type) {
            case SDL_QUIT:
                extern int running;
                running = 0;
                break;

            //Keyboard, Key press, and Key release events
            /*
            KeyPress(key)
            KeyRelease(key)
            Keyboard(key)   (Like KeyPress, but continuous)
            */
            case SDL_KEYDOWN:
                break;
            
            case SDL_KEYUP:
                break;
            
            //Mouse events
            /*
            MouseButtonPress(button)
            MouseButtonRelease(button)
            MouseButton(button) (Like MouseButtonPress, but continuous)
            MouseWheel(direction: {Up, Down})
            MouseEnter
            MouseLeave
            GlobalMouseButtonPress(button)
            GlobalMouseButtonRelease(button)
            GlobalMouseButton(button) (Like GlobalMouseButtonPress, but continuous)
            */
            case SDL_MOUSEBUTTONDOWN:
                break;
            
            case SDL_MOUSEBUTTONUP:
                break;
            
            default: break;
            }
        }

        //Normal step events (now all instances are set to their new positions)
        for(ObjectInstance &i: this->m_instances)
            i.m_generator->on_step(&i);

        //Collision events
        for(ObjectInstance &i1: this->m_instances)
            for(ObjectInstance &i2: this->m_instances) {
                if(&i2 == &i1)
                    continue;

                //TODO: Add BBox
            }

        //End step events
        for(ObjectInstance &i: this->m_instances)
            i.m_generator->on_end_step(&i);
    }

    void draw() {
        if(!curent_room)
            return;

        for(ObjectInstance &i: this->m_instances)
            i.m_generator->draw(&i);
    }

private:
    Room *curent_room;
    std::vector<ObjectInstance> m_instances;
    //std::vector<Tile> tiles;
};

extern RoomEnvironment current_room;

#endif /* _ROOM_HPP_ */
