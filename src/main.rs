use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const MAP_SIZE: usize = 16;

// Define a simple map (1 = wall, 0 = empty)
fn get_map() -> [[u8; MAP_SIZE]; MAP_SIZE] {
    [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]
}

fn main() {
    let mut window =
        Window::new("DOOM", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    let mut player_x = 2.5;
    let mut player_y = 2.5;

    // Direction vector
    let mut dir_x = 1.0;
    let mut dir_y = 0.0;

    // Camera plane (perpendicular to direction)
    let mut plane_x = 0.0;
    let mut plane_y = 0.77; // Field of view is roughly 2 * atan(0.77/1.0) ~= 77 degrees

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let move_speed = 0.05;
        let rotation_speed: f32 = 0.03;

        if window.is_key_down(Key::W) {
            let new_x = player_x + dir_x * move_speed;
            let new_y = player_y + dir_y * move_speed;

            // Collision detection
            let map = get_map();
            let cell_x = new_x as usize;
            let cell_y = new_y as usize;

            if cell_x < MAP_SIZE && cell_y < MAP_SIZE && map[cell_y][cell_x] == 0 {
                player_x = new_x;
                player_y = new_y;
            }
        }

        if window.is_key_down(Key::S) {
            let new_x = player_x - dir_x * move_speed;
            let new_y = player_y - dir_y * move_speed;

            // Collision detection
            let map = get_map();
            let cell_x = new_x as usize;
            let cell_y = new_y as usize;

            if cell_x < MAP_SIZE && cell_y < MAP_SIZE && map[cell_y][cell_x] == 0 {
                player_x = new_x;
                player_y = new_y;
            }
        }

        if window.is_key_down(Key::A) {
            let old_dir_x = dir_x;
            dir_x = dir_x * (-rotation_speed).cos() - dir_y * (-rotation_speed).sin();
            dir_y = old_dir_x * (-rotation_speed).sin() + dir_y * (-rotation_speed).cos();

            let old_plane_x = plane_x;
            plane_x = plane_x * (-rotation_speed).cos() - plane_y * (-rotation_speed).sin();
            plane_y = old_plane_x * (-rotation_speed).sin() + plane_y * (-rotation_speed).cos();
        }

        if window.is_key_down(Key::D) {
            let old_dir_x = dir_x;
            dir_x = dir_x * rotation_speed.cos() - dir_y * rotation_speed.sin();
            dir_y = old_dir_x * rotation_speed.sin() + dir_y * rotation_speed.cos();

            let old_plane_x = plane_x;
            plane_x = plane_x * rotation_speed.cos() - plane_y * rotation_speed.sin();
            plane_y = old_plane_x * rotation_speed.sin() + plane_y * rotation_speed.cos();
        }

        for y in 0..HEIGHT {
            if y < HEIGHT / 2 {
                // Ceiling color (dark gray)
                let ceiling_color = (30 << 16) | (30 << 8) | 30;
                for x in 0..WIDTH {
                    buffer[y * WIDTH + x] = ceiling_color;
                }
            } else {
                // Floor color (dark green)
                let floor_color = (30 << 16) | (50 << 8) | 30;
                for x in 0..WIDTH {
                    buffer[y * WIDTH + x] = floor_color;
                }
            }
        }

        // Cast rays
        let map = get_map();

        for x in 0..WIDTH {
            // Calculate ray position and direction
            // x-coordinate in camera space
            let camera_x = 2.0 * x as f32 / WIDTH as f32 - 1.0;
            let ray_dir_x = dir_x + plane_x * camera_x;
            let ray_dir_y = dir_y + plane_y * camera_x;

            // Which box of the map we are in
            let mut map_x = player_x as i32;
            let mut map_y = player_y as i32;

            // Length of ray from current position to next x or y side
            let mut side_dist_x;
            let mut side_dist_y;

            // Length of ray from one x or y-side to next x or y-side
            let delta_dist_x = if ray_dir_x.abs() < 1e-6 {
                f32::MAX
            } else {
                (1.0 / ray_dir_x).abs()
            };

            let delta_dist_y = if ray_dir_y.abs() < 1e-6 {
                f32::MAX
            } else {
                (1.0 / ray_dir_y).abs()
            };

            // What direction to step in x or y-direction (either +1 or -1)
            let step_x: i32;
            let step_y: i32;

            // Calculate step and initial side_dist
            if ray_dir_x < 0.0 {
                step_x = -1;
                side_dist_x = (player_x - map_x as f32) * delta_dist_x;
            } else {
                step_x = 1;
                side_dist_x = ((map_x as f32 + 1.0) - player_x) * delta_dist_x;
            }

            if ray_dir_y < 0.0 {
                step_y = -1;
                side_dist_y = (player_y - map_y as f32) * delta_dist_y;
            } else {
                step_y = 1;
                side_dist_y = ((map_y as f32 + 1.0) - player_y) * delta_dist_y;
            }

            // Perform DDA (Digital Differential Analysis)
            let mut hit = false;
            let mut side = 0; // Was a NS or a EW wall hit?
            let mut wall_type = 0;

            while !hit {
                // Jump to next map square, either in x-direction, or in y-direction
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                // Check if ray has hit a wall
                if map_y >= 0 && map_y < MAP_SIZE as i32 && map_x >= 0 && map_x < MAP_SIZE as i32 {
                    if map[map_y as usize][map_x as usize] > 0 {
                        hit = true;
                        wall_type = map[map_y as usize][map_x as usize];
                    }
                }
            }

            // Calculate distance projected on camera direction
            let perp_wall_dist = if side == 0 {
                (map_x as f32 - player_x + (1.0 - step_x as f32) / 2.0) / ray_dir_x
            } else {
                (map_y as f32 - player_y + (1.0 - step_y as f32) / 2.0) / ray_dir_y
            };

            // Calculate height of line to draw on screen
            let line_height = (HEIGHT as f32 / perp_wall_dist) as i32;

            // Calculate lowest and highest pixel to fill in current stripe
            let mut draw_start = -line_height / 2 + HEIGHT as i32 / 2;
            if draw_start < 0 {
                draw_start = 0;
            }

            let mut draw_end = line_height / 2 + HEIGHT as i32 / 2;
            if draw_end >= HEIGHT as i32 {
                draw_end = HEIGHT as i32 - 1;
            }

            // Choose wall color based on wall type and side
            let wall_color = match wall_type {
                1 => {
                    if side == 1 {
                        0xFF0000
                    } else {
                        0xCC0000
                    }
                } // Red walls
                2 => {
                    if side == 1 {
                        0x00FF00
                    } else {
                        0x00CC00
                    }
                } // Green walls
                3 => {
                    if side == 1 {
                        0x0000FF
                    } else {
                        0x0000CC
                    }
                } // Blue walls
                4 => {
                    if side == 1 {
                        0xFFFF00
                    } else {
                        0xCCCC00
                    }
                } // Yellow walls
                _ => {
                    if side == 1 {
                        0xFFFFFF
                    } else {
                        0xCCCCCC
                    }
                } // White walls
            };

            // Draw the pixels of the stripe
            for y in draw_start..draw_end {
                if y >= 0 && y < HEIGHT as i32 && x < WIDTH {
                    buffer[y as usize * WIDTH + x] = wall_color;
                }
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
