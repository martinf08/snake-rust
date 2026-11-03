use crate::board::{Board, Grid};
use crate::board_controller::BoardController;
use crate::config::GlobalConfig;
use crate::food::Food;
use crate::game_mode::GameMode;
use crate::portal::{Gate, Portal};
use crate::score::Score;
use crate::snake::Point;
use std::collections::LinkedList;
use std::sync::Arc;

fn config(mode: &str, wall: &str, size: u32) -> Arc<GlobalConfig> {
    Arc::new(GlobalConfig::from_toml(&format!(
        "[board]\nblock_size = 20\nboard_block_length = {}\n[game]\nmode = '{}'\nwall = '{}'\nlevel = 4",
        size, mode, wall,
    )).unwrap())
}

fn controller(mode: &str, wall: &str, size: u32) -> BoardController {
    let config = config(mode, wall, size);
    BoardController::new(Board::new(config.clone(), Arc::new(GameMode::new(config)), None), Score::new())
}

#[test]
fn grid_includes_every_playable_cell() {
    let grid = Grid::new(&400.0, &20.0);
    assert_eq!(grid.list.len(), 400);
    assert!(grid.list.contains(&(0.0, 0.0)));
    assert!(grid.list.contains(&(19.0, 19.0)));
}

#[test]
fn occupied_grid_excludes_fractional_body_food_and_gates() {
    let mut body = LinkedList::new();
    body.push_back(Point { x: 1.1, y: 1.0 });
    let grid = Grid::new(&60.0, &20.0).remove_occupied_positions(
        body, &Food { x: 2.0, y: 2.0 },
        Some(vec![Some(Gate { x: 0.0, y: 0.0, used: false })]),
    );
    assert_eq!(grid.list.len(), 6);
    assert!(!grid.list.contains(&(1.0, 1.0)));
    assert!(!grid.list.contains(&(2.0, 2.0)));
    assert!(!grid.list.contains(&(0.0, 0.0)));
}

#[test]
fn empty_grid_has_no_random_position() {
    assert!(Grid { list: vec![] }.get_random_position().is_none());
}

#[test]
fn invalid_configurations_are_rejected() {
    let valid = "[board]\nblock_size = 20\nboard_block_length = 20\n[game]\nmode = 'default'\nwall = 'solid'\nlevel = 4";
    for (from, to) in [
        ("block_size = 20", "block_size = 0"),
        ("board_block_length = 20", "board_block_length = 5"),
        ("level = 4", "level = 0"),
        ("level = 4", "level = 100"),
        ("'default'", "'unknown'"),
        ("'solid'", "'unknown'"),
    ].iter() {
        assert!(GlobalConfig::from_toml(&valid.replace(from, to)).is_err());
    }
    assert!(GlobalConfig::from_toml("broken toml").is_err());
}

#[test]
fn small_board_food_is_inside_grid_and_off_body() {
    for _ in 0..50 {
        let game = controller("portal", "fluid", 6);
        let food = game.board.food;
        assert!((0.0..6.0).contains(&food.x) && (0.0..6.0).contains(&food.y));
        assert!(!game.board.snake.body.iter().any(|p| (p.x.round(), p.y.round()) == (food.x, food.y)));
        let portal = game.board.portal.unwrap();
        assert_eq!(portal.gates.len(), 2);
        assert_ne!((portal.gates[0].x, portal.gates[0].y), (portal.gates[1].x, portal.gates[1].y));
        assert!(portal.gates.iter().all(|g| (g.x, g.y) != (food.x, food.y)));
    }
}

#[test]
fn portal_generation_handles_insufficient_space() {
    let mut game = controller("portal", "fluid", 6);
    game.board.grid.list.clear();
    assert!(Portal::new(&game.board).is_none());
    game.board.grid.list.push((0.0, 0.0));
    assert!(Portal::new(&game.board).is_none());
}

#[test]
fn timing_keeps_remainder_and_matches_split_updates() {
    let mut a = controller("default", "fluid", 20);
    let mut b = controller("default", "fluid", 20);
    a.board.food = Food { x: 0.0, y: 0.0 };
    b.board.food = a.board.food;
    a.update(0.05);
    for _ in 0..5 { b.update(0.01); }
    assert_eq!(a.board.snake.head_position(), b.board.snake.head_position());
    assert!((a.board.current_delta - b.board.current_delta).abs() < 1e-12);
    let before = b.board.snake.head_position();
    b.update(0.005);
    assert_eq!(before, b.board.snake.head_position());
}

#[test]
fn pause_freezes_simulation_and_invalid_dt_is_ignored() {
    let mut game = controller("default", "solid", 20);
    let before = game.board.snake.head_position();
    game.paused = true;
    game.update(1.0);
    game.paused = false;
    for dt in [f64::NAN, f64::INFINITY, -1.0, 0.0].iter() { game.update(*dt); }
    assert_eq!(before, game.board.snake.head_position());
}

#[test]
fn restart_preserves_best_and_does_not_add_default_mode_portals() {
    let mut game = controller("default", "solid", 20);
    game.score.update_score();
    game.paused = true;
    game.restart();
    assert!(game.board.portal.is_none());
    assert!(!game.paused);
    assert_eq!(game.score.scores["current"].count, 0);
    assert_eq!(game.score.scores["best"].count, 1);
    assert_eq!(game.score.scores["death"].count, 1);
}

#[test]
fn fluid_wall_does_not_disable_self_collision_at_board_edge() {
    let mut game = controller("default", "fluid", 20);
    game.board.snake.body.clear();
    for _ in 0..12 { game.board.snake.body.push_back(Point { x: 19.0, y: 3.0 }); }
    assert!(game.board.snake.is_dead(&400.0, &20.0));
}

#[test]
fn teleport_keeps_snake_length() {
    let mut game = controller("portal", "fluid", 20);
    let mut portal = game.board.portal.clone().unwrap();
    portal.gates[0].used = true;
    let length = game.board.snake.body.len();
    game.board.snake.teleport(portal);
    game.board.snake.update(1.0 / 60.0);
    assert_eq!(game.board.snake.body.len(), length);
}
