#ifndef _UTILS_HPP_
#define _UTILS_HPP_

struct Color {
    unsigned char r;
    unsigned char g;
    unsigned char b;
    unsigned char a;
};

struct Vector2 {
    int x;
    int y;

    Vector2& operator+=(Vector2 &rhs) {
        *this = *this + rhs;
        return *this;
    }

    Vector2& operator-=(Vector2 &rhs) {
        *this = *this - rhs;
        return *this;
    }

    Vector2& operator*=(Vector2 &rhs) {
        *this = *this * rhs;
        return *this;
    }

    Vector2& operator/=(Vector2 &rhs) {
        *this = *this / rhs;
        return *this;
    }

    Vector2& operator+=(int &rhs) {
        *this = *this + rhs;
        return *this;
    }

    Vector2& operator-=(int &rhs) {
        *this = *this - rhs;
        return *this;
    }

    Vector2& operator*=(int &rhs) {
        *this = *this * rhs;
        return *this;
    }

    Vector2& operator/=(int &rhs) {
        *this = *this / rhs;
        return *this;
    }


    Vector2 operator+(Vector2 &rhs) {
        return (Vector2) {
            this->x + rhs.x,
            this->y + rhs.y
        };
    }

    Vector2 operator-(Vector2 &rhs) {
        return (Vector2) {
            this->x - rhs.x,
            this->y - rhs.y
        };
    }

    Vector2 operator*(Vector2 &rhs) {
        return (Vector2) {
            this->x * rhs.x,
            this->y * rhs.y
        };
    }

    Vector2 operator/(Vector2 &rhs) {
        return (Vector2) {
            this->x / rhs.x,
            this->y / rhs.y
        };
    }

    Vector2 operator+(int rhs) {
        return (Vector2) {
            this->x + rhs,
            this->y + rhs
        };
    }

    Vector2 operator-(int rhs) {
        return (Vector2) {
            this->x - rhs,
            this->y - rhs
        };
    }

    Vector2 operator*(int rhs) {
        return (Vector2) {
            this->x * rhs,
            this->y * rhs
        };
    }

    Vector2 operator/(int rhs) {
        return (Vector2) {
            this->x / rhs,
            this->y / rhs
        };
    }
};

#endif /* _UTILS_HPP_ */
