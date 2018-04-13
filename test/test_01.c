#include <orbital.h>
#include <stdint.h>
#include <stdio.h>

#define WIDTH 640
#define HEIGHT 480

int main() {
    void *window = orb_window_new(100, 100, WIDTH, HEIGHT, "Test window");

    printf("Window size: %d x %d\n", orb_window_width(window), orb_window_height(window));

    int frame_count = 0;
    uint32_t *frame_data = orb_window_data(window);

    while (true) {
        for (int y = 0; y < HEIGHT; ++y) {
            for (int x = 0; x < WIDTH; ++x) {
                frame_data[y * WIDTH + x] =
                    0xFF000000 |
                    (((x ^ y ^ frame_count) & 0xFF) << 16) |
                    ((y & 0xFF) << 8) |
                    (x & 0xFF);
            }
        }

        orb_window_sync(window);
        ++frame_count;
    }

    return 0;
}
