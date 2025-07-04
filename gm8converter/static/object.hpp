#ifndef _OBJECT_HPP_
#define _OBJECT_HPP_

#include "alarm.hpp"
#include "sprite.hpp"

class Object;

struct ObjectInstance {
    ObjectInstance(const Object *generator, const Sprite *spr) :
        m_generator(generator),
        score(0),
        lives(0),
        health(100),     //TODO: Check
        visible(true),
        m_mark_for_deletion(false),
        m_sprite(spr->build_instance())
    {}

    void mark_for_deletion() {
        m_mark_for_deletion = true;
    }

    const Object *m_generator;

    int score;  // the current value of the score
    int lives;  // the current number of lives
    int health; // the current health (0-100)

    bool visible;
    Alarm m_alarms[12];

private:
    bool m_mark_for_deletion;
    SpriteInstance m_sprite;
};


class Object {
public:
    Object() :
        m_default_sprite(nullptr)
    {}

    void draw(ObjectInstance *i) const
    {
        if(!i->visible)
            return;

        on_draw(i);
        //TODO: Draw
    }
    
    void on_create(ObjectInstance *i) const {}
    void on_destroy(ObjectInstance *i) const {}
    void on_draw(ObjectInstance *i) const {}

    void on_begin_step(ObjectInstance *i) const {}
    void on_step(ObjectInstance *i) const {}
    void on_end_step(ObjectInstance *i) const {}
    void on_alarm(ObjectInstance *i, int alarm_id) const {}

    ObjectInstance build_instance()  const {
        ObjectInstance output(this, this->m_default_sprite);
        //init_instance(&output);
        return output;
    }

    const Sprite* default_sprite() const
    { return m_default_sprite; }

private:
    const Sprite *m_default_sprite;
};

#endif /* _OBJECT_HPP_ */
