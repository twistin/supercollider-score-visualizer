// =================================================================
// 🎵 SC SCORE VISUALIZER - SISTEMA DE MENÚS
// =================================================================
// Interfaz de menús profesional para la aplicación

use nannou::prelude::*;
use crate::model::{Model, MenuType, ColorTheme, UiState};

// =================================================================
// CONSTANTES DE MENÚ
// =================================================================

const MENU_HEIGHT: f32 = 30.0;
const MENU_ITEM_PADDING: f32 = 15.0;
const SUBMENU_WIDTH: f32 = 200.0;
const SUBMENU_ITEM_HEIGHT: f32 = 25.0;

// =================================================================
// FUNCIÓN PRINCIPAL DE DIBUJO DEL MENÚ
// =================================================================

pub fn draw_menu_bar(draw: &Draw, model: &Model, window_rect: Rect) {
    if !model.ui_state.show_menu {
        return;
    }
    
    // Fondo de la barra de menú
    let menu_bg = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.95, 0.95, 0.95, 0.95),
        ColorTheme::Dark => rgba(0.1, 0.1, 0.15, 0.95),
        ColorTheme::Blue => rgba(0.05, 0.1, 0.2, 0.95),
        ColorTheme::Classic => rgba(0.2, 0.2, 0.2, 0.95),
    };
    
    let menu_rect = Rect::from_x_y_w_h(
        window_rect.x(),
        window_rect.top() - MENU_HEIGHT / 2.0,
        window_rect.w(),
        MENU_HEIGHT,
    );
    
    draw.rect()
        .xy(menu_rect.xy())
        .wh(menu_rect.wh())
        .color(menu_bg);
    
    // Borde inferior del menú
    let border_color = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.7, 0.7, 0.7, 0.8),
        _ => rgba(0.3, 0.6, 1.0, 0.6),
    };
    
    draw.line()
        .start(pt2(window_rect.left(), window_rect.top() - MENU_HEIGHT))
        .end(pt2(window_rect.right(), window_rect.top() - MENU_HEIGHT))
        .color(border_color)
        .weight(1.0);
    
    // Título de la aplicación
    let title_color = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.2, 0.2, 0.4, 1.0),
        _ => rgba(0.8, 0.9, 1.0, 1.0),
    };
    
    draw.text("🎵 SC Score Visualizer")
        .xy(pt2(window_rect.left() + 120.0, window_rect.top() - MENU_HEIGHT / 2.0))
        .font_size(14)
        .color(title_color)
        .center_justify();
    
    // Elementos del menú
    let menu_items = [
        ("File", MenuType::File),
        ("Edit", MenuType::Edit),
        ("Display", MenuType::Display),
        ("View", MenuType::View),
    ];
    
    let mut x_offset = window_rect.left() + 280.0;
    
    for (label, menu_type) in menu_items.iter() {
        let is_active = model.ui_state.menu_state.active_menu.as_ref() == Some(menu_type);
        let text_color = if is_active {
            rgba(1.0, 1.0, 1.0, 1.0)
        } else {
            match model.ui_state.theme {
                ColorTheme::Light => rgba(0.3, 0.3, 0.5, 0.9),
                _ => rgba(0.7, 0.8, 0.9, 0.9),
            }
        };
        
        // Fondo del item activo
        if is_active {
            let item_bg = rgba(0.3, 0.6, 1.0, 0.3);
            draw.rect()
                .xy(pt2(x_offset + MENU_ITEM_PADDING, window_rect.top() - MENU_HEIGHT / 2.0))
                .wh(pt2(MENU_ITEM_PADDING * 2.0, MENU_HEIGHT - 4.0))
                .color(item_bg);
        }
        
        draw.text(label)
            .xy(pt2(x_offset + MENU_ITEM_PADDING, window_rect.top() - MENU_HEIGHT / 2.0))
            .font_size(12)
            .color(text_color)
            .center_justify();
        
        x_offset += MENU_ITEM_PADDING * 2.0 + 20.0;
    }
    
    // Indicadores de estado en la esquina derecha
    draw_status_indicators(draw, model, window_rect);
    
    // Dibujar submenús si están activos
    if let Some(active_menu) = &model.ui_state.menu_state.active_menu {
        draw_submenu(draw, model, window_rect, active_menu);
    }
}

// =================================================================
// INDICADORES DE ESTADO
// =================================================================

fn draw_status_indicators(draw: &Draw, model: &Model, window_rect: Rect) {
    let mut x_pos = window_rect.right() - 20.0;
    let y_pos = window_rect.top() - MENU_HEIGHT / 2.0;
    
    let indicator_color = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.3, 0.6, 0.3, 0.8),
        _ => rgba(0.3, 1.0, 0.3, 0.8),
    };
    
    // Indicador de alta resolución
    if model.ui_state.high_resolution {
        draw.text("HD")
            .xy(pt2(x_pos, y_pos))
            .font_size(10)
            .color(indicator_color)
            .center_justify();
        x_pos -= 30.0;
    }
    
    // Indicador de modo performance
    if model.ui_state.performance_mode {
        draw.text("⚡")
            .xy(pt2(x_pos, y_pos))
            .font_size(12)
            .color(indicator_color)
            .center_justify();
        x_pos -= 25.0;
    }
    
    // Indicador de fullscreen
    if model.ui_state.fullscreen {
        draw.text("⛶")
            .xy(pt2(x_pos, y_pos))
            .font_size(12)
            .color(indicator_color)
            .center_justify();
        x_pos -= 25.0;
    }
    
    // Nivel de zoom
    if model.ui_state.zoom_level != 1.0 {
        let zoom_text = format!("{:.1}x", model.ui_state.zoom_level);
        draw.text(&zoom_text)
            .xy(pt2(x_pos, y_pos))
            .font_size(10)
            .color(indicator_color)
            .center_justify();
    }
}

// =================================================================
// SUBMENÚS
// =================================================================

fn draw_submenu(draw: &Draw, model: &Model, window_rect: Rect, menu_type: &MenuType) {
    let menu_items = get_menu_items(menu_type);
    let submenu_height = menu_items.len() as f32 * SUBMENU_ITEM_HEIGHT + 10.0;
    
    // Posición del submenú
    let submenu_x = match menu_type {
        MenuType::File => window_rect.left() + 280.0,
        MenuType::Edit => window_rect.left() + 330.0,
        MenuType::Display => window_rect.left() + 380.0,
        MenuType::View => window_rect.left() + 460.0,
    };
    
    let submenu_y = window_rect.top() - MENU_HEIGHT - submenu_height / 2.0;
    
    // Fondo del submenú
    let submenu_bg = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.98, 0.98, 0.98, 0.98),
        ColorTheme::Dark => rgba(0.08, 0.08, 0.12, 0.98),
        ColorTheme::Blue => rgba(0.03, 0.08, 0.15, 0.98),
        ColorTheme::Classic => rgba(0.15, 0.15, 0.15, 0.98),
    };
    
    draw.rect()
        .xy(pt2(submenu_x + SUBMENU_WIDTH / 2.0, submenu_y))
        .wh(pt2(SUBMENU_WIDTH, submenu_height))
        .color(submenu_bg);
    
    // Borde del submenú
    let border_color = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.6, 0.6, 0.6, 0.8),
        _ => rgba(0.3, 0.6, 1.0, 0.6),
    };
    
    draw.rect()
        .xy(pt2(submenu_x + SUBMENU_WIDTH / 2.0, submenu_y))
        .wh(pt2(SUBMENU_WIDTH, submenu_height))
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(border_color);
    
    // Items del submenú
    for (i, item) in menu_items.iter().enumerate() {
        let item_y = window_rect.top() - MENU_HEIGHT - 15.0 - (i as f32 * SUBMENU_ITEM_HEIGHT);
        let is_enabled = is_menu_item_enabled(item, model);
        
        let text_color = if is_enabled {
            match model.ui_state.theme {
                ColorTheme::Light => rgba(0.2, 0.2, 0.4, 0.9),
                _ => rgba(0.9, 0.9, 0.95, 0.9),
            }
        } else {
            match model.ui_state.theme {
                ColorTheme::Light => rgba(0.5, 0.5, 0.5, 0.6),
                _ => rgba(0.5, 0.5, 0.5, 0.6),
            }
        };
        
        // Separadores
        if item.starts_with("---") {
            draw.line()
                .start(pt2(submenu_x + 10.0, item_y))
                .end(pt2(submenu_x + SUBMENU_WIDTH - 10.0, item_y))
                .color(rgba(0.5, 0.5, 0.5, 0.3))
                .weight(1.0);
            continue;
        }
        
        draw.text(item)
            .xy(pt2(submenu_x + 15.0, item_y))
            .font_size(11)
            .color(text_color)
            .left_justify();
        
        // Indicador de estado para items con estado
        draw_menu_item_indicator(draw, item, model, submenu_x + SUBMENU_WIDTH - 20.0, item_y);
    }
}

fn get_menu_items(menu_type: &MenuType) -> Vec<&'static str> {
    match menu_type {
        MenuType::File => vec![
            "New Session",
            "Open Session...",
            "Save Session",
            "Save As...",
            "---",
            "Export Image...",
            "Export Video...",
            "---",
            "Preferences...",
            "---",
            "Quit"
        ],
        MenuType::Edit => vec![
            "Undo",
            "Redo",
            "---",
            "Clear Events",
            "Reset Settings",
            "---",
            "Copy Screenshot",
            "---",
            "Find..."
        ],
        MenuType::Display => vec![
            "Fullscreen",
            "Performance Mode",
            "---",
            "Show Timer",
            "Show Grid",
            "Show Axis Labels",
            "---",
            "Snap to X-Grid",
            "Snap to Y-Grid",
            "---",
            "Grid Resolution +",
            "Grid Resolution -",
            "---",
            "High Resolution",
            "---",
            "Light Theme",
            "Dark Theme",
            "Blue Theme",
            "Classic Theme"
        ],
        MenuType::View => vec![
            "Zoom In",
            "Zoom Out",
            "Reset Zoom",
            "---",
            "Fit to Window",
            "Resize Viewport",
            "---",
            "Show Statistics",
            "Show Trails",
            "---",
            "Musical Mode",
            "Linear Mode"
        ],
    }
}

fn is_menu_item_enabled(item: &str, _model: &Model) -> bool {
    match item {
        "Undo" | "Redo" | "Find..." => false, // Not implemented yet
        _ => true,
    }
}

fn draw_menu_item_indicator(draw: &Draw, item: &str, model: &Model, x: f32, y: f32) {
    let indicator = match item {
        "Fullscreen" if model.ui_state.fullscreen => "✓",
        "Performance Mode" if model.ui_state.performance_mode => "✓",
        "Show Timer" if model.ui_state.show_timer => "✓",
        "Show Grid" if model.ui_state.show_grid => "✓",
        "Show Axis Labels" if model.config.visual.grid.show_labels => "✓",
        "Snap to X-Grid" if model.ui_state.snap_to_x_grid => "✓",
        "Snap to Y-Grid" if model.ui_state.snap_to_y_grid => "✓",
        "High Resolution" if model.ui_state.high_resolution => "✓",
        "Show Statistics" if model.ui_state.show_stats => "✓",
        "Show Trails" if model.ui_state.show_trails => "✓",
        "Musical Mode" if model.config.visual.grid.musical_divisions => "✓",
        "Light Theme" if model.ui_state.theme == ColorTheme::Light => "✓",
        "Dark Theme" if model.ui_state.theme == ColorTheme::Dark => "✓",
        "Blue Theme" if model.ui_state.theme == ColorTheme::Blue => "✓",
        "Classic Theme" if model.ui_state.theme == ColorTheme::Classic => "✓",
        _ => return,
    };
    
    let indicator_color = match model.ui_state.theme {
        ColorTheme::Light => rgba(0.3, 0.6, 0.3, 0.8),
        _ => rgba(0.3, 1.0, 0.3, 0.8),
    };
    
    draw.text(indicator)
        .xy(pt2(x, y))
        .font_size(10)
        .color(indicator_color)
        .center_justify();
}

// =================================================================
// UTILIDADES DE MENÚ
// =================================================================

pub fn get_menu_rect(window_rect: Rect) -> Rect {
    Rect::from_x_y_w_h(
        window_rect.x(),
        window_rect.top() - MENU_HEIGHT / 2.0,
        window_rect.w(),
        MENU_HEIGHT,
    )
}

pub fn handle_menu_click(model: &mut Model, click_pos: Vec2, window_rect: Rect) -> bool {
    if !model.ui_state.show_menu {
        return false;
    }
    
    let menu_rect = get_menu_rect(window_rect);
    if !menu_rect.contains(click_pos) {
        model.ui_state.menu_state.active_menu = None;
        return false;
    }
    
    // Detectar clic en elementos del menú principal
    let menu_items = [
        ("File", MenuType::File, 280.0),
        ("Edit", MenuType::Edit, 330.0),
        ("Display", MenuType::Display, 380.0),
        ("View", MenuType::View, 460.0),
    ];
    
    for (_, menu_type, x_pos) in menu_items.iter() {
        let item_rect = Rect::from_x_y_w_h(
            window_rect.left() + x_pos + MENU_ITEM_PADDING,
            window_rect.top() - MENU_HEIGHT / 2.0,
            MENU_ITEM_PADDING * 2.0,
            MENU_HEIGHT - 4.0,
        );
        
        if item_rect.contains(click_pos) {
            if model.ui_state.menu_state.active_menu.as_ref() == Some(menu_type) {
                model.ui_state.menu_state.active_menu = None;
            } else {
                model.ui_state.menu_state.active_menu = Some(menu_type.clone());
            }
            return true;
        }
    }
    
    true
}

pub fn handle_submenu_click(model: &mut Model, click_pos: Vec2, window_rect: Rect) -> bool {
    if let Some(active_menu) = &model.ui_state.menu_state.active_menu {
        let menu_items = get_menu_items(active_menu);
        let submenu_x = match active_menu {
            MenuType::File => window_rect.left() + 280.0,
            MenuType::Edit => window_rect.left() + 330.0,
            MenuType::Display => window_rect.left() + 380.0,
            MenuType::View => window_rect.left() + 460.0,
        };
        
        for (i, item) in menu_items.iter().enumerate() {
            let item_y = window_rect.top() - MENU_HEIGHT - 15.0 - (i as f32 * SUBMENU_ITEM_HEIGHT);
            let item_rect = Rect::from_x_y_w_h(
                submenu_x + SUBMENU_WIDTH / 2.0,
                item_y,
                SUBMENU_WIDTH,
                SUBMENU_ITEM_HEIGHT,
            );
            
            if item_rect.contains(click_pos) && !item.starts_with("---") {
                execute_menu_action(model, item);
                model.ui_state.menu_state.active_menu = None;
                return true;
            }
        }
    }
    false
}

fn execute_menu_action(model: &mut Model, action: &str) {
    match action {
        // File menu actions
        "New Session" => {
            model.events.clear();
            model.ui_state = UiState::default();
            println!("🆕 Nueva sesión creada");
        },
        "Open Session..." => {
            if let Err(e) = model.load_session("session.toml") {
                eprintln!("❌ Error cargando sesión: {}", e);
            }
        },
        "Save Session" => {
            if let Err(e) = model.save_session("session.toml") {
                eprintln!("❌ Error guardando sesión: {}", e);
            } else {
                println!("💾 Sesión guardada");
            }
        },
        "Save As..." => {
            if let Err(e) = model.save_session("session_backup.toml") {
                eprintln!("❌ Error guardando sesión: {}", e);
            } else {
                println!("💾 Sesión guardada como backup");
            }
        },
        "Export Image..." => {
            if let Err(e) = model.export_image("visualization.png") {
                eprintln!("❌ Error exportando imagen: {}", e);
            } else {
                println!("🖼️ Imagen exportada");
            }
        },
        "Export Video..." => {
            if let Err(e) = model.export_video("visualization.mp4") {
                eprintln!("❌ Error exportando video: {}", e);
            } else {
                println!("🎥 Video exportado");
            }
        },
        "Preferences..." => {
            model.show_preferences();
        },
        "Quit" => {
            println!("👋 Cerrando aplicación...");
            std::process::exit(0);
        },
        
        // Edit menu actions
        "Clear Events" => {
            model.events.clear();
            println!("🗑️ Eventos limpiados");
        },
        "Reset Settings" => {
            model.reset_settings();
        },
        "Copy Screenshot" => {
            if let Err(e) = model.copy_screenshot() {
                eprintln!("❌ Error copiando captura: {}", e);
            } else {
                println!("📋 Captura copiada al portapapeles");
            }
        },
        
        // Display menu actions
        "Fullscreen" => model.toggle_ui_element("fullscreen"),
        "Performance Mode" => model.toggle_ui_element("performance"),
        "Show Timer" => model.toggle_ui_element("timer"),
        "Show Grid" => model.toggle_ui_element("grid"),
        "Show Axis Labels" => model.config.visual.grid.show_labels = !model.config.visual.grid.show_labels,
        "Snap to X-Grid" => model.toggle_ui_element("snap_x"),
        "Snap to Y-Grid" => model.toggle_ui_element("snap_y"),
        "High Resolution" => model.toggle_ui_element("high_res"),
        "Grid Resolution +" => {
            model.adjust_grid_resolution(2);
            println!("📏 Resolución de rejilla: {}", model.ui_state.grid_resolution);
        },
        "Grid Resolution -" => {
            model.adjust_grid_resolution(-2);
            println!("📏 Resolución de rejilla: {}", model.ui_state.grid_resolution);
        },
        "Light Theme" => {
            model.change_theme(ColorTheme::Light);
            println!("🌞 Tema claro activado");
        },
        "Dark Theme" => {
            model.change_theme(ColorTheme::Dark);
            println!("🌙 Tema oscuro activado");
        },
        "Blue Theme" => {
            model.change_theme(ColorTheme::Blue);
            println!("🔵 Tema azul activado");
        },
        "Classic Theme" => {
            model.change_theme(ColorTheme::Classic);
            println!("⚫ Tema clásico activado");
        },
        
        // View menu actions
        "Zoom In" => {
            model.zoom_in();
            println!("🔍 Zoom: {:.1}x", model.ui_state.zoom_level);
        },
        "Zoom Out" => {
            model.zoom_out();
            println!("🔍 Zoom: {:.1}x", model.ui_state.zoom_level);
        },
        "Reset Zoom" => {
            model.reset_zoom();
            println!("🔍 Zoom reiniciado");
        },
        "Fit to Window" => {
            model.fit_to_window();
            println!("📏 Ajustado a ventana");
        },
        "Resize Viewport" => {
            model.resize_viewport(Vec2::new(1200.0, 800.0));
        },
        "Show Statistics" => model.toggle_ui_element("stats"),
        "Show Trails" => model.toggle_ui_element("trails"),
        "Musical Mode" => {
            model.config.visual.grid.musical_divisions = true;
            println!("🎵 Modo musical activado");
        },
        "Linear Mode" => {
            model.config.visual.grid.musical_divisions = false;
            println!("📐 Modo lineal activado");
        },
        
        _ => {
            println!("⚠️ Acción no implementada: {}", action);
        }
    }
}
