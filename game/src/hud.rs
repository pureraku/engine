use crate::ai::AiManager;
use crate::arena::Arena;
use crate::pickups::PickupManager;
use crate::player::Player;
use crate::weapons::Weapons;
use engine::{UiRenderer, Vec4};

pub struct Hud {
    pub hitmarker_timer: f32,
}

impl Hud {
    pub fn new() -> Self {
        Self {
            hitmarker_timer: 0.0,
        }
    }

    pub fn trigger_hitmarker(&mut self) {
        self.hitmarker_timer = 0.14;
    }

    pub fn update(&mut self, dt: f32) {
        self.hitmarker_timer = (self.hitmarker_timer - dt).max(0.0);
    }

    pub fn draw(
        &self,
        ui: &mut UiRenderer,
        player: &Player,
        weapons: &Weapons,
        ai: &AiManager,
        pickups: &PickupManager,
        arena: &Arena,
        time: f32,
    ) {
        let sw = ui.screen_width;
        let sh = ui.screen_height;
        let cx = sw * 0.5;
        let cy = sh * 0.5;

        // 1. Damage Screen Flash
        if player.damage_flash > 0.0 {
            let alpha = (player.damage_flash / 0.35).clamp(0.0, 0.45);
            ui.draw_rect(0.0, 0.0, sw, sh, Vec4::new(1.0, 0.0, 0.0, alpha));
        }

        // Low health pulsing edge vignette
        if player.health < 30.0 && player.is_alive() {
            let pulse = (time * 6.0).sin() * 0.5 + 0.5;
            let border_t = 12.0;
            let col = Vec4::new(0.9, 0.1, 0.1, 0.25 * pulse);
            ui.draw_rect_outline(0.0, 0.0, sw, sh, border_t, col);
        }

        // 2. Crosshair & Hitmarker (if player is alive)
        if player.is_alive() {
            let ch_col = Vec4::new(0.0, 1.0, 0.8, 0.85);
            let gap = if weapons.fire_timer > 0.0 { 8.0 } else { 5.0 };
            ui.draw_crosshair(cx, cy, 9.0, gap, 2.0, ch_col);

            if self.hitmarker_timer > 0.0 {
                let hm_col = Vec4::new(1.0, 0.2, 0.2, 0.95);
                ui.draw_hitmarker(cx, cy, 8.0, 2.0, hm_col);
            }
        }

        // 3. Top-Center Status: Wave, Score, Kills, Remaining Enemies
        let top_w = 420.0;
        let top_h = 44.0;
        let top_x = cx - top_w * 0.5;
        let top_y = 12.0;

        ui.draw_rect(top_x, top_y, top_w, top_h, Vec4::new(0.05, 0.07, 0.12, 0.75));
        ui.draw_rect_outline(top_x, top_y, top_w, top_h, 1.5, Vec4::new(0.0, 0.6, 0.9, 0.6));

        let wave_text = format!("WAVE {:02}", ai.current_wave);
        ui.draw_text(top_x + 16.0, top_y + 14.0, &wave_text, 2.0, Vec4::new(1.0, 0.85, 0.2, 1.0));

        let enemies_text = format!("HOSTILES: {}", ai.enemies.len());
        ui.draw_text(top_x + 130.0, top_y + 14.0, &enemies_text, 2.0, Vec4::new(1.0, 0.3, 0.3, 1.0));

        let kills_text = format!("KILLS: {:02}", ai.total_kills);
        ui.draw_text(top_x + 270.0, top_y + 14.0, &kills_text, 2.0, Vec4::new(0.2, 0.9, 1.0, 1.0));

        // Sub-bar for score
        let score_text = format!("SCORE: {:06}", ai.score);
        ui.draw_text(top_x + 140.0, top_y + 50.0, &score_text, 2.0, Vec4::new(0.9, 0.9, 0.9, 0.9));

        // 4. Bottom-Left Vitality (Health & Armor)
        let vit_w = 260.0;
        let vit_h = 76.0;
        let vit_x = 24.0;
        let vit_y = sh - vit_h - 24.0;

        ui.draw_rect(vit_x, vit_y, vit_w, vit_h, Vec4::new(0.05, 0.07, 0.12, 0.8));
        ui.draw_rect_outline(vit_x, vit_y, vit_w, vit_h, 1.5, Vec4::new(0.1, 0.7, 0.9, 0.7));

        // Health Bar
        let hp = player.health.max(0.0) as i32;
        let hp_ratio = player.health / player.max_health;
        let hp_col = if hp_ratio > 0.5 {
            Vec4::new(0.1, 0.95, 0.3, 0.9)
        } else if hp_ratio > 0.25 {
            Vec4::new(0.95, 0.75, 0.1, 0.9)
        } else {
            Vec4::new(0.95, 0.15, 0.15, 0.9)
        };

        let hp_text = format!("HP {:03}", hp);
        ui.draw_text(vit_x + 12.0, vit_y + 12.0, &hp_text, 2.0, hp_col);
        ui.draw_bar(
            vit_x + 85.0,
            vit_y + 12.0,
            160.0,
            14.0,
            player.health,
            player.max_health,
            hp_col,
            Vec4::new(0.15, 0.15, 0.2, 0.6),
            Vec4::new(0.3, 0.4, 0.5, 0.8),
        );

        // Armor Bar
        let ap = player.armor.max(0.0) as i32;
        let ap_col = Vec4::new(0.1, 0.8, 1.0, 0.9);
        let ap_text = format!("AP {:03}", ap);
        ui.draw_text(vit_x + 12.0, vit_y + 42.0, &ap_text, 2.0, ap_col);
        ui.draw_bar(
            vit_x + 85.0,
            vit_y + 42.0,
            160.0,
            14.0,
            player.armor,
            player.max_armor,
            ap_col,
            Vec4::new(0.15, 0.15, 0.2, 0.6),
            Vec4::new(0.3, 0.4, 0.5, 0.8),
        );

        // 5. Bottom-Right Arsenal (Active Weapon & Ammunition)
        let ars_w = 260.0;
        let ars_h = 76.0;
        let ars_x = sw - ars_w - 24.0;
        let ars_y = sh - ars_h - 24.0;

        ui.draw_rect(ars_x, ars_y, ars_w, ars_h, Vec4::new(0.05, 0.07, 0.12, 0.8));
        ui.draw_rect_outline(ars_x, ars_y, ars_w, ars_h, 1.5, Vec4::new(0.1, 0.7, 0.9, 0.7));

        let active_w = weapons.active();
        let name_col = Vec4::new(active_w.color.x, active_w.color.y, active_w.color.z, 1.0);
        let num_key = weapons.active_index + 1;
        let title_text = format!("{}. {}", num_key, active_w.name);
        ui.draw_text(ars_x + 12.0, ars_y + 12.0, &title_text, 2.0, name_col);

        if weapons.is_reloading {
            ui.draw_text(ars_x + 12.0, ars_y + 42.0, "RELOADING...", 2.0, Vec4::new(1.0, 0.8, 0.1, 1.0));
        } else {
            let ammo_text = format!("{:02} / {:03}", active_w.ammo_in_mag, active_w.ammo_reserve);
            let ammo_col = if active_w.ammo_in_mag == 0 {
                Vec4::new(1.0, 0.2, 0.2, 1.0)
            } else {
                Vec4::new(0.9, 0.95, 1.0, 1.0)
            };
            ui.draw_text(ars_x + 12.0, ars_y + 42.0, &ammo_text, 3.0, ammo_col);
        }

        // Weapon Selector Pips (1 2 3)
        for i in 0..3 {
            let pip_x = ars_x + ars_w - 60.0 + (i as f32) * 16.0;
            let pip_y = ars_y + 46.0;
            let is_sel = i == weapons.active_index;
            let col = if is_sel {
                Vec4::new(0.2, 0.9, 1.0, 1.0)
            } else {
                Vec4::new(0.3, 0.35, 0.4, 0.6)
            };
            ui.draw_rect(pip_x, pip_y, 10.0, 10.0, col);
        }

        // 6. Top-Right Tactical Radar / Minimap
        let radar_size = 120.0;
        let radar_x = sw - radar_size - 24.0;
        let radar_y = 24.0;

        ui.draw_rect(radar_x, radar_y, radar_size, radar_size, Vec4::new(0.04, 0.06, 0.1, 0.75));
        ui.draw_rect_outline(radar_x, radar_y, radar_size, radar_size, 1.5, Vec4::new(0.1, 0.6, 0.8, 0.6));

        // Radar center
        let rc_x = radar_x + radar_size * 0.5;
        let rc_y = radar_y + radar_size * 0.5;
        let scale = (radar_size * 0.45) / arena.arena_half_size;

        // Draw arena boundary outline on radar
        let b_size = arena.arena_half_size * 2.0 * scale;
        ui.draw_rect_outline(rc_x - b_size * 0.5, rc_y - b_size * 0.5, b_size, b_size, 1.0, Vec4::new(0.2, 0.3, 0.4, 0.5));

        // Draw radar blips:
        // Pickups (yellow/green dots)
        for (_, pos) in pickups.active_pickups() {
            let bx = rc_x + pos.x * scale;
            let by = rc_y + pos.z * scale;
            ui.draw_rect(bx - 1.5, by - 1.5, 3.0, 3.0, Vec4::new(0.2, 1.0, 0.4, 0.8));
        }

        // Enemies (red blinking dots)
        for e in &ai.enemies {
            let bx = rc_x + e.position.x * scale;
            let by = rc_y + e.position.z * scale;
            let col = match e.enemy_type {
                crate::ai::EnemyType::Titan => Vec4::new(1.0, 0.0, 1.0, 0.95),
                _ => Vec4::new(1.0, 0.2, 0.2, 0.95),
            };
            ui.draw_rect(bx - 2.0, by - 2.0, 4.0, 4.0, col);
        }

        // Player (bright cyan arrow / dot with direction tick)
        let px = rc_x + player.position.x * scale;
        let py = rc_y + player.position.z * scale;
        ui.draw_rect(px - 2.5, py - 2.5, 5.0, 5.0, Vec4::new(0.0, 1.0, 1.0, 1.0));
        let yaw_rad = player.yaw.to_radians();
        let dir_tick_x = px + yaw_rad.cos() * 6.0;
        let dir_tick_y = py + yaw_rad.sin() * 6.0;
        ui.draw_rect(dir_tick_x - 1.0, dir_tick_y - 1.0, 2.0, 2.0, Vec4::new(0.0, 1.0, 1.0, 0.9));

        // 7. Intermission or Game Over / Victory Banners
        if !player.is_alive() {
            // Game Over overlay
            ui.draw_rect(0.0, 0.0, sw, sh, Vec4::new(0.1, 0.0, 0.0, 0.65));
            let banner_w = 460.0;
            let banner_h = 130.0;
            let bx = cx - banner_w * 0.5;
            let by = cy - banner_h * 0.5 - 20.0;

            ui.draw_rect(bx, by, banner_w, banner_h, Vec4::new(0.08, 0.05, 0.05, 0.9));
            ui.draw_rect_outline(bx, by, banner_w, banner_h, 2.0, Vec4::new(1.0, 0.2, 0.2, 0.9));

            ui.draw_text(bx + 40.0, by + 24.0, "PROTOCOL TERMINATED", 3.0, Vec4::new(1.0, 0.2, 0.2, 1.0));
            ui.draw_text(bx + 60.0, by + 65.0, "SECTOR OVERRUN - YOU DIED", 2.0, Vec4::new(0.9, 0.7, 0.7, 0.9));
            ui.draw_text(bx + 75.0, by + 95.0, "PRESS [R] TO RESTART", 2.0, Vec4::new(1.0, 0.9, 0.2, 1.0));
        } else if ai.wave_cleared {
            // Wave Cleared banner
            let banner_w = 380.0;
            let banner_h = 60.0;
            let bx = cx - banner_w * 0.5;
            let by = cy - 100.0;

            ui.draw_rect(bx, by, banner_w, banner_h, Vec4::new(0.05, 0.1, 0.08, 0.85));
            ui.draw_rect_outline(bx, by, banner_w, banner_h, 2.0, Vec4::new(0.2, 0.9, 0.4, 0.85));

            let clear_text = format!("WAVE {} CLEARED!", ai.current_wave);
            ui.draw_text(bx + 55.0, by + 16.0, &clear_text, 3.0, Vec4::new(0.2, 1.0, 0.4, 1.0));
            ui.draw_text(bx + 85.0, by + 40.0, "INCOMING WAVE PREPARING...", 1.5, Vec4::new(0.8, 0.9, 0.8, 0.9));
        }
    }
}
