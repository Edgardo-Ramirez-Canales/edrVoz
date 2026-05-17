use super::{scaled, LOGICAL_H, SUPER_SAMPLE};

pub(super) fn draw_control_shadow(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: f32,
) {
    for spread in (1..=10).rev() {
        let alpha = (22 - spread).max(0) as u8;
        draw_round_rect(
            pixels,
            width,
            height,
            x - spread,
            y + spread / 3,
            w + spread * 2,
            h + spread,
            radius + spread as f32,
            [0, 0, 0, alpha],
        );
    }
}

pub(super) fn draw_control_body(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: f32,
) {
    draw_round_rect(pixels, width, height, x, y, w, h, radius, [10, 10, 10, 242]);
    draw_round_rect_stroke(
        pixels,
        width,
        height,
        x,
        y,
        w,
        h,
        radius,
        1,
        [255, 255, 255, 22],
    );
    draw_round_rect(
        pixels,
        width,
        height,
        x + 3,
        y + 2,
        w - 6,
        (h / 2).max(1),
        radius,
        [255, 255, 255, 8],
    );
}

pub(super) fn draw_button_shell(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    cx: i32,
    cy: i32,
    radius: i32,
) {
    for spread in (1..=9).rev() {
        let alpha = (20 - spread).max(0) as u8;
        draw_circle(
            pixels,
            width,
            height,
            cx,
            cy + spread / 3,
            radius + spread,
            [0, 0, 0, alpha],
        );
    }
    draw_circle(pixels, width, height, cx, cy, radius, [10, 10, 10, 242]);
    draw_circle(pixels, width, height, cx, cy, radius, [255, 255, 255, 20]);
    draw_circle(pixels, width, height, cx, cy, radius - 1, [10, 10, 10, 242]);
    draw_circle(
        pixels,
        width,
        height,
        cx,
        cy - radius / 3,
        radius / 2,
        [255, 255, 255, 8],
    );
}

fn draw_round_rect(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: f32,
    color: [u8; 4],
) {
    for py in y.max(0)..(y + h).min(height) {
        for px in x.max(0)..(x + w).min(width) {
            let coverage = round_rect_coverage(px, py, x, y, w, h, radius);
            if coverage > 0.0 {
                blend_pixel(pixels, width, px, py, color, coverage);
            }
        }
    }
}

fn draw_round_rect_stroke(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    radius: f32,
    thickness: i32,
    color: [u8; 4],
) {
    draw_round_rect(pixels, width, height, x, y, w, h, radius, color);
    draw_round_rect(
        pixels,
        width,
        height,
        x + thickness,
        y + thickness,
        w - thickness * 2,
        h - thickness * 2,
        (radius - thickness as f32).max(0.0),
        [18, 18, 18, 235],
    );
}

pub(super) fn draw_button_x(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    cx: i32,
    cy: i32,
    scale: f64,
) {
    draw_line(
        pixels,
        width,
        height,
        cx - scaled(3, scale),
        cy - scaled(3, scale),
        cx + scaled(3, scale),
        cy + scaled(3, scale),
        scaled(1, scale).max(1),
        [145, 153, 166, 220],
    );
    draw_line(
        pixels,
        width,
        height,
        cx + scaled(3, scale),
        cy - scaled(3, scale),
        cx - scaled(3, scale),
        cy + scaled(3, scale),
        scaled(1, scale).max(1),
        [145, 153, 166, 220],
    );
}

pub(super) fn draw_button_check(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    cx: i32,
    cy: i32,
    scale: f64,
    active: bool,
) {
    if active {
        draw_circle(
            pixels,
            width,
            height,
            cx,
            cy,
            scaled(11, scale),
            [255, 255, 255, 238],
        );
    }
    let fg = if active {
        [18, 18, 18, 245]
    } else {
        [145, 153, 166, 170]
    };
    draw_line(
        pixels,
        width,
        height,
        cx - scaled(4, scale),
        cy,
        cx - scaled(1, scale),
        cy + scaled(3, scale),
        scaled(1, scale).max(1),
        fg,
    );
    draw_line(
        pixels,
        width,
        height,
        cx - scaled(1, scale),
        cy + scaled(3, scale),
        cx + scaled(5, scale),
        cy - scaled(5, scale),
        scaled(1, scale).max(1),
        fg,
    );
}

pub(super) fn draw_wave(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    center_x: i32,
    center_w: i32,
    scale: f64,
    alpha: u8,
    frame: u32,
) {
    let center_y = scaled(LOGICAL_H / 2, scale);
    let point_count = 25;
    let padding = scaled(9, scale);
    let wave_left = center_x + padding;
    let wave_right = center_x + center_w - padding;
    let span = (wave_right - wave_left).max(1) as f32;
    for i in 0..point_count {
        let t = i as f32 / (point_count - 1) as f32;
        let x = wave_left + (span * t).round() as i32;
        let offset = i as f32 - (point_count - 1) as f32 / 2.0;
        let distance = (offset.abs() / ((point_count - 1) as f32 / 2.0)).clamp(0.0, 1.0);
        let envelope = 0.35 + (1.0 - distance) * 0.65;
        let phase = (frame as f32 * 0.72) + (i as f32 * 0.55);
        let amp = (phase.sin() * 0.5 + 0.5).powf(1.4);
        let logical_h = if frame == 0 {
            3.0
        } else {
            3.0 + amp * 13.0 * envelope
        };
        let dot_alpha = if frame == 0 {
            alpha
        } else {
            ((alpha as f32) * (0.42 + amp * 0.48 + envelope * 0.10)).round() as u8
        };
        let h = scaled(logical_h.round() as i32, scale).max(2);
        draw_round_rect(
            pixels,
            width,
            height,
            x,
            center_y - h / 2,
            scaled(2, scale).max(1),
            h,
            scaled(1, scale) as f32,
            [255, 255, 255, dot_alpha],
        );
    }
}

pub(super) fn draw_ready_mark(pixels: &mut [u8], width: i32, height: i32, scale: f64) {
    let cx = scaled(130, scale);
    let cy = scaled(LOGICAL_H / 2, scale);
    draw_line(
        pixels,
        width,
        height,
        cx - scaled(10, scale),
        cy,
        cx - scaled(4, scale),
        cy + scaled(6, scale),
        scaled(2, scale).max(1),
        [74, 222, 128, 245],
    );
    draw_line(
        pixels,
        width,
        height,
        cx - scaled(4, scale),
        cy + scaled(6, scale),
        cx + scaled(12, scale),
        cy - scaled(10, scale),
        scaled(2, scale).max(1),
        [74, 222, 128, 245],
    );
}

pub(super) fn draw_error_mark(pixels: &mut [u8], width: i32, height: i32, scale: f64) {
    let cx = scaled(130, scale);
    let cy = scaled(LOGICAL_H / 2, scale);
    draw_circle(
        pixels,
        width,
        height,
        cx,
        cy,
        scaled(14, scale),
        [245, 158, 11, 50],
    );
    draw_line(
        pixels,
        width,
        height,
        cx,
        cy - scaled(8, scale),
        cx,
        cy + scaled(3, scale),
        scaled(3, scale).max(2),
        [251, 191, 36, 255],
    );
    draw_circle(
        pixels,
        width,
        height,
        cx,
        cy + scaled(9, scale),
        scaled(2, scale).max(1),
        [251, 191, 36, 255],
    );
}

fn draw_circle(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    cx: i32,
    cy: i32,
    radius: i32,
    color: [u8; 4],
) {
    let r = radius as f32;
    for y in (cy - radius - 1).max(0)..(cy + radius + 1).min(height) {
        for x in (cx - radius - 1).max(0)..(cx + radius + 1).min(width) {
            let dx = x as f32 + 0.5 - cx as f32;
            let dy = y as f32 + 0.5 - cy as f32;
            let coverage = (r + 0.5 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            if coverage > 0.0 {
                blend_pixel(pixels, width, x, y, color, coverage);
            }
        }
    }
}

fn draw_line(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    thickness: i32,
    color: [u8; 4],
) {
    let min_x = x0.min(x1) - thickness - 1;
    let max_x = x0.max(x1) + thickness + 1;
    let min_y = y0.min(y1) - thickness - 1;
    let max_y = y0.max(y1) + thickness + 1;
    let ax = x0 as f32;
    let ay = y0 as f32;
    let bx = x1 as f32;
    let by = y1 as f32;
    let abx = bx - ax;
    let aby = by - ay;
    let ab_len2 = abx * abx + aby * aby;
    let radius = thickness as f32 / 2.0;

    for y in min_y.max(0)..max_y.min(height) {
        for x in min_x.max(0)..max_x.min(width) {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let t = (((px - ax) * abx + (py - ay) * aby) / ab_len2).clamp(0.0, 1.0);
            let closest_x = ax + abx * t;
            let closest_y = ay + aby * t;
            let dx = px - closest_x;
            let dy = py - closest_y;
            let coverage = (radius + 0.5 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            if coverage > 0.0 {
                blend_pixel(pixels, width, x, y, color, coverage);
            }
        }
    }
}

fn round_rect_coverage(px: i32, py: i32, x: i32, y: i32, w: i32, h: i32, radius: f32) -> f32 {
    let mut inside = 0;
    let step = 1.0 / SUPER_SAMPLE as f32;
    for sy in 0..SUPER_SAMPLE {
        for sx in 0..SUPER_SAMPLE {
            let sample_x = px as f32 + (sx as f32 + 0.5) * step;
            let sample_y = py as f32 + (sy as f32 + 0.5) * step;
            if point_in_round_rect(sample_x, sample_y, x, y, w, h, radius) {
                inside += 1;
            }
        }
    }
    inside as f32 / (SUPER_SAMPLE * SUPER_SAMPLE) as f32
}

fn point_in_round_rect(px: f32, py: f32, x: i32, y: i32, w: i32, h: i32, radius: f32) -> bool {
    let min_x = x as f32;
    let min_y = y as f32;
    let max_x = (x + w) as f32;
    let max_y = (y + h) as f32;
    if px < min_x || py < min_y || px >= max_x || py >= max_y {
        return false;
    }

    let radius = radius.min(w as f32 / 2.0).min(h as f32 / 2.0).max(0.0);
    let clamp_min_x = min_x + radius;
    let clamp_max_x = max_x - radius;
    let clamp_min_y = min_y + radius;
    let clamp_max_y = max_y - radius;
    let cx = if clamp_min_x <= clamp_max_x {
        px.clamp(clamp_min_x, clamp_max_x)
    } else {
        (min_x + max_x) / 2.0
    };
    let cy = if clamp_min_y <= clamp_max_y {
        py.clamp(clamp_min_y, clamp_max_y)
    } else {
        (min_y + max_y) / 2.0
    };
    let dx = px - cx;
    let dy = py - cy;
    dx * dx + dy * dy <= radius * radius
}

fn blend_pixel(pixels: &mut [u8], width: i32, x: i32, y: i32, color: [u8; 4], coverage: f32) {
    let idx = ((y * width + x) * 4) as usize;
    let src_a = (color[3] as f32 / 255.0) * coverage;
    if src_a <= 0.0 {
        return;
    }

    let dst_a = pixels[idx + 3] as f32 / 255.0;
    let out_a = src_a + dst_a * (1.0 - src_a);
    if out_a <= 0.0 {
        return;
    }

    for channel in 0..3 {
        let src_channel = match channel {
            0 => 2,
            1 => 1,
            _ => 0,
        };
        let src = color[src_channel] as f32 / 255.0;
        let dst = if dst_a > 0.0 {
            pixels[idx + channel] as f32 / 255.0 / dst_a
        } else {
            0.0
        };
        let out = (src * src_a + dst * dst_a * (1.0 - src_a)) / out_a;
        pixels[idx + channel] = (out * out_a * 255.0).round().clamp(0.0, 255.0) as u8;
    }
    pixels[idx + 3] = (out_a * 255.0).round().clamp(0.0, 255.0) as u8;
}
