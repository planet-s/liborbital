#include <orbital.h>
#include <stdint.h>
#include <stdio.h>

#define WIDTH 640
#define HEIGHT 480

int main() {
    void *window = orb_window_new(10, 10, 320, 240, "Test window");

    orb_window_set_pos(window, 100, 100);
    orb_window_set_size(window, WIDTH, HEIGHT);

    printf("Display size: (%d, %d)\n", orb_display_width(), orb_display_height());
    printf("Window size: (%d, %d)\n", orb_window_width(window), orb_window_height(window));
    printf("Window position: (%d, %d)\n", orb_window_x(window), orb_window_y(window));

    char title[1024] = { 0 };
    int frame_count = 0;
    uint32_t *frame_data = orb_window_data(window);

    while (true) {
        sprintf(title, "Frame #%d", frame_count);
        orb_window_set_title(window, title);

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
