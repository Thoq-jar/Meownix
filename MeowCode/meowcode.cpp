#include <gtk/gtk.h>
#include <gtk/gtktextview.h>
#include <algorithm>
#include <cctype>
#include <string>
#include <glib.h>
#include <glib-object.h>
#include <iostream>

static gchar *last_opened_file = NULL;

extern "C" {
    static void open_file(GtkWidget *, gpointer);
    static void save_file(GtkWidget *, gpointer);
    static void quit_application(GtkWidget *, gpointer);
    static void window_clicked(GtkWidget *, GdkEventButton *, gpointer);
    static void show_about_dialog(GtkWidget *, gpointer);
}

GdkPixbuf *create_pixbuf(const gchar *filename) {
  GdkPixbuf *pixbuf;
  GError *error = NULL;
  pixbuf = gdk_pixbuf_new_from_file(filename, &error);
  if (!pixbuf) {
    fprintf(stderr, "%s\n", error->message);
    g_error_free(error);
  }
  return pixbuf;
}

static void update_last_opened_file(const gchar *filename) {
    if (last_opened_file != NULL) {
        g_free(last_opened_file);
    }
    last_opened_file = g_strdup(filename);
}

static void open_file(GtkWidget *widget, gpointer text_view) {
    GtkWidget *dialog;
    GtkTextBuffer *buffer;
    GtkFileChooserAction action = GTK_FILE_CHOOSER_ACTION_OPEN;
    gint res;

    dialog = gtk_file_chooser_dialog_new(
        "Open File", GTK_WINDOW(gtk_widget_get_toplevel(widget)), action,
        "_Cancel", GTK_RESPONSE_CANCEL, "_Open", GTK_RESPONSE_ACCEPT, NULL);

    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
        char *filename;
        GtkTextIter start, end;

        GtkFileChooser *chooser = GTK_FILE_CHOOSER(dialog);
        filename = gtk_file_chooser_get_filename(chooser);

        buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_view));
        gtk_text_buffer_get_start_iter(buffer, &start);
        gtk_text_buffer_get_end_iter(buffer, &end);
        gtk_text_buffer_delete(buffer, &start, &end);

        gchar *contents;
        gsize length;

        if (g_file_get_contents(filename, &contents, &length, NULL)) {
            gtk_text_buffer_insert_at_cursor(buffer, contents, length);
            g_free(contents);
            update_last_opened_file(filename);
        }

        g_free(filename);
    }

    gtk_widget_destroy(dialog);
}

static void save_file(GtkWidget *widget, gpointer text_view) {
    GtkWidget *dialog;
    GtkFileChooserAction action = GTK_FILE_CHOOSER_ACTION_SAVE;
    gint res;

    dialog = gtk_file_chooser_dialog_new(
        "Save File", GTK_WINDOW(gtk_widget_get_toplevel(widget)), action,
        "_Cancel", GTK_RESPONSE_CANCEL, "_Save", GTK_RESPONSE_ACCEPT, NULL);

    if (last_opened_file != NULL) {
        gtk_file_chooser_set_current_name(GTK_FILE_CHOOSER(dialog), last_opened_file);
    }

    res = gtk_dialog_run(GTK_DIALOG(dialog));
    if (res == GTK_RESPONSE_ACCEPT) {
        char *filename;
        GtkTextIter start, end;
        GtkTextBuffer *buffer;

        GtkFileChooser *chooser = GTK_FILE_CHOOSER(dialog);
        filename = gtk_file_chooser_get_filename(chooser);

        buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(text_view));
        gtk_text_buffer_get_bounds(buffer, &start, &end);
        gchar *text = gtk_text_buffer_get_text(buffer, &start, &end, FALSE);

        GError *error = NULL;
        if (!g_file_set_contents(filename, text, -1, &error)) {
            g_warning("%s", error->message);
            g_error_free(error);
        }

        g_free(text);
        g_free(filename);
    }

    gtk_widget_destroy(dialog);
}

static void quit_application(GtkWidget *widget, gpointer data) {
  gtk_main_quit();
}

static gboolean on_key_press(GtkWidget *widget, GdkEventKey *event, gpointer data) {
    GtkTextBuffer *buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(widget));
    GtkTextIter iter, start, end;
    GtkTextMark *insert_mark, *selection_bound;

    if ((event->keyval == GDK_KEY_s || event->keyval == GDK_KEY_S) &&
        (event->state & GDK_CONTROL_MASK)) {
        save_file(widget, data);
        return TRUE; 
    }

    if ((event->keyval == GDK_KEY_o || event->keyval == GDK_KEY_O) &&
        (event->state & GDK_CONTROL_MASK)) {
        open_file(widget, data);
        return TRUE;
    }

    if ((event->keyval == GDK_KEY_q || event->keyval == GDK_KEY_Q) &&
        (event->state & GDK_CONTROL_MASK)) {
        quit_application(widget, data);
        return TRUE;
    }

    if (event->state & GDK_CONTROL_MASK) {
        gtk_text_buffer_get_iter_at_mark(buffer, &iter, gtk_text_buffer_get_insert(buffer));
        gtk_text_buffer_get_start_iter(buffer, &start);
        gtk_text_buffer_get_end_iter(buffer, &end);
        gtk_text_buffer_select_range(buffer, &start, &end);
    }

    return FALSE;
}

static void window_clicked(GtkWidget *widget, GdkEventButton *event,
                           gpointer data) {
  gtk_window_present(GTK_WINDOW(widget));
}

static void show_about_dialog(GtkWidget *widget, gpointer data) {
  GtkWidget *about_dialog = gtk_about_dialog_new();
  gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(about_dialog),
                                    "MeowCode - About");
  gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(about_dialog), "(Meow)");
  gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(about_dialog),
                                 "Copyright © 2024-Present Meownix");
  gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(about_dialog),
                                "Code at the speed of light.");
  gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(about_dialog), "about:blank");
  gtk_about_dialog_set_website_label(GTK_ABOUT_DIALOG(about_dialog),
                                     "Coming Soon");
  gtk_about_dialog_set_license(
      GTK_ABOUT_DIALOG(about_dialog),
      "Thoq License - (Custom Licesne) - "
      "https://raw.githubusercontent.com/Thoq-jar/Meownix-Utils/main/licence");
  gtk_about_dialog_set_logo(GTK_ABOUT_DIALOG(about_dialog),
                            gdk_pixbuf_new_from_file("", NULL));
  gtk_dialog_run(GTK_DIALOG(about_dialog));
  gtk_widget_destroy(about_dialog);
}

static gboolean on_motion_notify(GtkWidget *widget, GdkEventMotion *event,
                                 gpointer data) {
  GtkTextBuffer *buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(widget));
  GtkTextIter start, end;

  double x, y;

  GdkDevice *device = gdk_event_get_device((const GdkEvent *)event);

  if (!device) {
    g_warning("Invalid GdkDevice in motion notify event");
    return FALSE;
  }

  gdk_window_get_device_position_double(gtk_widget_get_window(widget), device,
                                        &x, &y, NULL);

  return FALSE;
}

static gboolean select_all_text(GtkWidget *widget, GdkEventKey *event,
                                gpointer data) {
  GtkTextBuffer *buffer = gtk_text_view_get_buffer(GTK_TEXT_VIEW(widget));
  GtkTextIter start, end;

  if (event->keyval == GDK_KEY_a && (event->state & GDK_CONTROL_MASK)) {
    gtk_text_buffer_get_start_iter(buffer, &start);
    gtk_text_buffer_get_end_iter(buffer, &end);
    gtk_text_buffer_select_range(buffer, &start, &end);
    return TRUE;
  }

  return FALSE;
}

int main(int argc, char *argv[]) {
  gtk_init(&argc, &argv);

  GtkWidget *text_view = gtk_text_view_new();
  gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(text_view), GTK_WRAP_WORD_CHAR);

  g_signal_connect(G_OBJECT(text_view), "key_press_event",
                   G_CALLBACK(on_key_press), text_view);

  GtkWidget *scrolled_window = gtk_scrolled_window_new(NULL, NULL);
  gtk_widget_set_size_request(scrolled_window, 1600, 900);
  gtk_container_add(GTK_CONTAINER(scrolled_window), text_view);

  GtkWidget *window = gtk_window_new(GTK_WINDOW_TOPLEVEL);
  gtk_window_set_default_size(GTK_WINDOW(window), 1600, 900);
  g_signal_connect(window, "destroy", G_CALLBACK(gtk_main_quit), NULL);

  GtkWidget *box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
  gtk_box_pack_start(GTK_BOX(box), scrolled_window, TRUE, TRUE, 0);
  GtkWidget *menu_bar = gtk_menu_bar_new();
  GtkWidget *file_menu = gtk_menu_new();
  GtkWidget *file_menu_item = gtk_menu_item_new_with_label("File");
  gtk_menu_item_set_submenu(GTK_MENU_ITEM(file_menu_item), file_menu);
  gtk_menu_shell_append(GTK_MENU_SHELL(menu_bar), file_menu_item);

  gtk_window_set_title(GTK_WINDOW(window), "MeowCode");
  GtkWidget *open_item = gtk_menu_item_new_with_label("Open");
  gtk_menu_shell_append(GTK_MENU_SHELL(file_menu), open_item);
  g_signal_connect(open_item, "activate", G_CALLBACK(open_file), text_view);

  GtkWidget *save_item = gtk_menu_item_new_with_label("Save");
  gtk_menu_shell_append(GTK_MENU_SHELL(file_menu), save_item);
  g_signal_connect(save_item, "activate", G_CALLBACK(save_file), text_view);


  GtkWidget *quit_item = gtk_menu_item_new_with_label("Quit");
  gtk_menu_shell_append(GTK_MENU_SHELL(file_menu), quit_item);
  g_signal_connect(quit_item, "activate", G_CALLBACK(gtk_main_quit), NULL);

  GtkWidget *about_item = gtk_menu_item_new_with_label("About");
  gtk_menu_shell_append(GTK_MENU_SHELL(file_menu), about_item);
  g_signal_connect(about_item, "activate", G_CALLBACK(show_about_dialog), NULL);

  gtk_box_pack_start(GTK_BOX(box), menu_bar, FALSE, FALSE, 0);

  gtk_container_add(GTK_CONTAINER(window), box);

  const char* cssStyles = "* {"
                        "background-color: rgba(10, 10, 10, 1);"
                        "color: white;"
                        "border: none;"
                        "font-size: 15px;"
                        "font-family: sans-serif;"
                        "border: none;"
                        "}";
  GtkCssProvider *css_provider = gtk_css_provider_new();
  GError *error = NULL;
  gtk_css_provider_load_from_data(css_provider, cssStyles, -1, &error);
  if (error!= NULL) {
    g_printerr("Failed to load CSS: %s\n", error? error->message : "Unknown error");
    g_clear_error(&error);
  } else {
    gtk_style_context_add_provider_for_screen(gdk_screen_get_default(),
                                              GTK_STYLE_PROVIDER(css_provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
  }
  
  gtk_widget_show_all(window);
  gtk_main();

  return 0;
}
