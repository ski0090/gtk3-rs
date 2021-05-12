var N = null;var sourcesIndex = {};
sourcesIndex["atk"] = {"name":"","dirs":[{"name":"auto","files":["action.rs","alias.rs","component.rs","document.rs","editable_text.rs","enums.rs","flags.rs","gobject_accessible.rs","hyperlink.rs","hyperlink_impl.rs","hypertext.rs","image.rs","misc.rs","mod.rs","no_op_object.rs","no_op_object_factory.rs","object.rs","object_factory.rs","plug.rs","range.rs","rectangle.rs","registry.rs","relation.rs","relation_set.rs","selection.rs","socket.rs","state_set.rs","streamable_content.rs","table.rs","table_cell.rs","text.rs","text_range.rs","util.rs","value.rs","window.rs"]}],"files":["attribute.rs","attribute_set.rs","editable_text.rs","lib.rs","prelude.rs","rt.rs","table.rs","text_rectangle.rs"]};
sourcesIndex["atk_sys"] = {"name":"","files":["lib.rs"]};
sourcesIndex["gdk"] = {"name":"","dirs":[{"name":"auto","files":["app_launch_context.rs","cursor.rs","device.rs","device_manager.rs","device_pad.rs","device_tool.rs","display.rs","display_manager.rs","drag_context.rs","drawing_context.rs","enums.rs","event_sequence.rs","flags.rs","frame_clock.rs","frame_timings.rs","functions.rs","gl_context.rs","keymap.rs","mod.rs","monitor.rs","screen.rs","seat.rs","visual.rs","window.rs"]}],"files":["atom.rs","cairo_interaction.rs","change_data.rs","device.rs","device_manager.rs","drag_context.rs","event.rs","event_button.rs","event_configure.rs","event_crossing.rs","event_dnd.rs","event_expose.rs","event_focus.rs","event_grab_broken.rs","event_key.rs","event_motion.rs","event_owner_change.rs","event_pad_axis.rs","event_pad_button.rs","event_pad_group_mode.rs","event_property.rs","event_proximity.rs","event_scroll.rs","event_selection.rs","event_setting.rs","event_touch.rs","event_touchpad_pinch.rs","event_touchpad_swipe.rs","event_visibility.rs","event_window_state.rs","frame_clock.rs","frame_timings.rs","functions.rs","geometry.rs","keymap.rs","keymap_key.rs","keys.rs","lib.rs","prelude.rs","rectangle.rs","rgba.rs","rt.rs","screen.rs","time_coord.rs","visual.rs","window.rs"]};
sourcesIndex["gdk_sys"] = {"name":"","files":["lib.rs"]};
sourcesIndex["gdk_x11_sys"] = {"name":"","files":["lib.rs","manual.rs"]};
sourcesIndex["gdkx11"] = {"name":"","dirs":[{"name":"auto","files":["functions.rs","mod.rs","x11_app_launch_context.rs","x11_cursor.rs","x11_device_core.rs","x11_device_manager_core.rs","x11_device_manager_xi2.rs","x11_device_xi2.rs","x11_display.rs","x11_display_manager.rs","x11_drag_context.rs","x11_keymap.rs","x11_monitor.rs","x11_screen.rs","x11_visual.rs","x11_window.rs","x11gl_context.rs"]}],"files":["lib.rs","rt.rs"]};
sourcesIndex["gtk"] = {"name":"","dirs":[{"name":"auto","files":["about_dialog.rs","accel_group.rs","accel_label.rs","action_bar.rs","actionable.rs","adjustment.rs","app_chooser_button.rs","app_chooser_dialog.rs","app_chooser_widget.rs","application.rs","application_window.rs","aspect_frame.rs","assistant.rs","bin.rs","box_.rs","buildable.rs","builder.rs","button.rs","button_box.rs","calendar.rs","cell_area.rs","cell_area_box.rs","cell_area_context.rs","cell_editable.rs","cell_layout.rs","cell_renderer.rs","cell_renderer_accel.rs","cell_renderer_combo.rs","cell_renderer_pixbuf.rs","cell_renderer_progress.rs","cell_renderer_spin.rs","cell_renderer_spinner.rs","cell_renderer_text.rs","cell_renderer_toggle.rs","cell_view.rs","check_button.rs","check_menu_item.rs","clipboard.rs","color_button.rs","color_chooser.rs","color_chooser_dialog.rs","color_chooser_widget.rs","combo_box.rs","combo_box_text.rs","constants.rs","container.rs","css_provider.rs","css_section.rs","dialog.rs","drawing_area.rs","editable.rs","entry.rs","entry_completion.rs","enums.rs","event_box.rs","event_controller.rs","event_controller_key.rs","event_controller_motion.rs","event_controller_scroll.rs","expander.rs","file_chooser.rs","file_chooser_button.rs","file_chooser_dialog.rs","file_chooser_native.rs","file_chooser_widget.rs","file_filter.rs","fixed.rs","flags.rs","flow_box.rs","flow_box_child.rs","font_button.rs","font_chooser.rs","font_chooser_dialog.rs","font_chooser_widget.rs","frame.rs","functions.rs","gesture.rs","gesture_drag.rs","gesture_long_press.rs","gesture_multi_press.rs","gesture_pan.rs","gesture_rotate.rs","gesture_single.rs","gesture_stylus.rs","gesture_swipe.rs","gesture_zoom.rs","gl_area.rs","grid.rs","header_bar.rs","icon_info.rs","icon_theme.rs","icon_view.rs","im_context.rs","im_context_simple.rs","im_multicontext.rs","image.rs","info_bar.rs","invisible.rs","label.rs","layout.rs","level_bar.rs","link_button.rs","list_box.rs","list_box_row.rs","list_store.rs","lock_button.rs","menu.rs","menu_bar.rs","menu_button.rs","menu_item.rs","menu_shell.rs","menu_tool_button.rs","message_dialog.rs","misc.rs","mod.rs","model_button.rs","mount_operation.rs","native_dialog.rs","notebook.rs","offscreen_window.rs","orientable.rs","overlay.rs","pad_controller.rs","page_setup.rs","paned.rs","paper_size.rs","places_sidebar.rs","plug.rs","popover.rs","popover_menu.rs","print_context.rs","print_operation.rs","print_operation_preview.rs","print_settings.rs","progress_bar.rs","radio_button.rs","radio_menu_item.rs","radio_tool_button.rs","range.rs","recent_chooser.rs","recent_chooser_dialog.rs","recent_chooser_menu.rs","recent_chooser_widget.rs","recent_filter.rs","recent_info.rs","recent_manager.rs","revealer.rs","scale.rs","scale_button.rs","scrollable.rs","scrollbar.rs","scrolled_window.rs","search_bar.rs","search_entry.rs","selection_data.rs","separator.rs","separator_menu_item.rs","separator_tool_item.rs","settings.rs","shortcut_label.rs","shortcuts_group.rs","shortcuts_section.rs","shortcuts_shortcut.rs","shortcuts_window.rs","size_group.rs","socket.rs","spin_button.rs","spinner.rs","stack.rs","stack_sidebar.rs","stack_switcher.rs","statusbar.rs","style_context.rs","style_properties.rs","style_provider.rs","switch.rs","target_list.rs","text_attributes.rs","text_buffer.rs","text_child_anchor.rs","text_iter.rs","text_mark.rs","text_tag.rs","text_tag_table.rs","text_view.rs","toggle_button.rs","toggle_tool_button.rs","tool_button.rs","tool_item.rs","tool_item_group.rs","tool_palette.rs","tool_shell.rs","toolbar.rs","tooltip.rs","tree_drag_dest.rs","tree_drag_source.rs","tree_iter.rs","tree_model.rs","tree_model_filter.rs","tree_model_sort.rs","tree_path.rs","tree_row_reference.rs","tree_selection.rs","tree_sortable.rs","tree_store.rs","tree_view.rs","tree_view_column.rs","viewport.rs","volume_button.rs","widget.rs","widget_path.rs","window.rs","window_group.rs"]},{"name":"subclass","files":["application.rs","application_window.rs","bin.rs","box_.rs","button.rs","cell_renderer.rs","cell_renderer_accel.rs","cell_renderer_combo.rs","cell_renderer_pixbuf.rs","cell_renderer_progress.rs","cell_renderer_spin.rs","cell_renderer_spinner.rs","cell_renderer_text.rs","cell_renderer_toggle.rs","container.rs","dialog.rs","drawing_area.rs","event_box.rs","fixed.rs","header_bar.rs","icon_view.rs","list_box.rs","list_box_row.rs","mod.rs","plug.rs","socket.rs","stack.rs","tree_view.rs","widget.rs","window.rs"]}],"files":["accel_group.rs","app_chooser.rs","application.rs","application_window.rs","border.rs","buildable.rs","builder.rs","cell_renderer_pixbuf.rs","clipboard.rs","color_button.rs","color_chooser.rs","combo_box.rs","dialog.rs","drag_context.rs","entry.rs","entry_buffer.rs","entry_completion.rs","enums.rs","file_chooser_dialog.rs","fixed.rs","flow_box.rs","gesture_stylus.rs","im_context_simple.rs","invisible.rs","lib.rs","list_box.rs","list_store.rs","menu.rs","message_dialog.rs","native_dialog.rs","notebook.rs","pad_action_entry.rs","pad_controller.rs","page_range.rs","prelude.rs","print_settings.rs","radio_button.rs","radio_menu_item.rs","radio_tool_button.rs","recent_chooser_dialog.rs","recent_data.rs","requisition.rs","response_type.rs","rt.rs","selection_data.rs","signal.rs","style_context.rs","switch.rs","target_entry.rs","target_list.rs","text_buffer.rs","text_iter.rs","tree_model_filter.rs","tree_path.rs","tree_row_reference.rs","tree_sortable.rs","tree_store.rs","widget.rs","window.rs","xlib.rs"]};
sourcesIndex["gtk3_macros"] = {"name":"","files":["attribute_parser.rs","composite_template_derive.rs","lib.rs","util.rs"]};
sourcesIndex["gtk_sys"] = {"name":"","files":["lib.rs","manual.rs"]};
createSourceSidebar();
