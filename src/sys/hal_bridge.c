#include <stdarg.h>
#include <stdlib.h>

#include "../../vendor/tamalib/hal.h"
#include "../../vendor/tamalib/tamalib.h"

extern void rs_hal_halt(void);
extern bool_t rs_hal_is_log_enabled(int level);
extern void rs_hal_sleep_until(timestamp_t ts);
extern timestamp_t rs_hal_get_timestamp(void);
extern void rs_hal_update_screen(void);
extern void rs_hal_set_lcd_matrix(u8_t x, u8_t y, bool_t val);
extern void rs_hal_set_lcd_icon(u8_t icon, bool_t val);
extern void rs_hal_set_frequency(u32_t freq);
extern void rs_hal_play_frequency(bool_t enabled);
extern int rs_hal_handler(void);

static bool_t bridge_is_log_enabled(log_level_t level) {
    return rs_hal_is_log_enabled((int)level);
}

static void bridge_log(log_level_t level, char *buff, ...) {
    (void)level;
    (void)buff;
}

static void *bridge_malloc(u32_t size) {
    return malloc(size);
}

static void bridge_free(void *ptr) {
    free(ptr);
}

static hal_t g_bridge_hal = {
    bridge_malloc,
    bridge_free,
    rs_hal_halt,
    bridge_is_log_enabled,
    bridge_log,
    rs_hal_sleep_until,
    rs_hal_get_timestamp,
    rs_hal_update_screen,
    rs_hal_set_lcd_matrix,
    rs_hal_set_lcd_icon,
    rs_hal_set_frequency,
    rs_hal_play_frequency,
    rs_hal_handler,
};

void tamars_register_hal(void) {
    tamalib_register_hal(&g_bridge_hal);
}
