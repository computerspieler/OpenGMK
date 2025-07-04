#include "sprite.hpp"

void SpriteInstance::update() {
    current_keyframe = (current_keyframe + 1) % m_src->keyframe_count();
}

void SpriteInstance::draw() {
    
}