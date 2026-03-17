#include "./str_buf.h"
#include <gtk/gtk.h>
#include <stdlib.h>
#include <string.h>

#define INIT_CAP 128

StrBuf str_buf_new() {
  StrBuf buf;

  buf.ptr = (char *)calloc(INIT_CAP, 1);
  buf.len = 0;
  buf.cap = INIT_CAP;

  return buf;
}

void str_buf_delete(StrBuf *buf) { free(buf->ptr); }

void str_buf_set_str(StrBuf *buf, KimeRustStr s) {
  if (s.len >= buf->cap) {
    size_t new_cap = s.len + 1;
    char *new_ptr = realloc(buf->ptr, new_cap);
    if (!new_ptr) {
      return;
    }
    buf->ptr = new_ptr;
    buf->cap = new_cap;
  }

  memcpy(buf->ptr, s.ptr, s.len);
  buf->len = s.len;
  buf->ptr[s.len] = '\0';
}

void str_buf_set_ch(StrBuf *buf, uint32_t ch) {
  // g_unichar_to_utf8 writes up to 6 bytes; ensure capacity (6 + null)
  if (buf->cap < 7) {
    char *new_ptr = realloc(buf->ptr, 7);
    if (!new_ptr) {
      return;
    }
    buf->ptr = new_ptr;
    buf->cap = 7;
  }
  gint len = g_unichar_to_utf8(ch, buf->ptr);
  buf->ptr[len] = '\0';
  buf->len = len;
}
