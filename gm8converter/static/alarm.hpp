#ifndef _ALARM_HPP_
#define _ALARM_HPP_

#include <algorithm>
class Alarm {
public:
    Alarm():
        m_time_left(0),
        m_already_fired_event(true)
    {}

    void update() {
        if(!m_time_left) {
            if(!m_already_fired_event)
                m_already_fired_event = true;
            return;
        }

        m_time_left --;
    }

    void set_time(int t) {
        m_time_left = std::max(t, 0); //TODO: Check how to behave when m_time_left = 0
        m_already_fired_event = false;
    }

    void add_time(int t) {
        m_time_left = std::max(m_time_left+t, 0); //TODO: Check how to behave when m_time_left = 0
        m_already_fired_event = false;
    }

    bool fire_event() const {
        return m_time_left <= 0 && !m_already_fired_event;
    }

private:
    int m_time_left;
    bool m_already_fired_event;
};

#endif /* _ALARM_HPP_ */
