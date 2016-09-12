#define NK_IMPLEMENTATION
#define NK_INCLUDE_FIXED_TYPES
#define NK_INCLUDE_COMMAND_USERDATA
#define NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#define NK_INCLUDE_FONT_BAKING

#include "nuklear/nuklear.h"

extern int nk_filter_my(const struct nk_text_edit *box, nk_rune unicode) {
	return 0;
}
