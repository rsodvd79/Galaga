use galaga_core::{
    audio::SoundEffect,
    entities::*,
    game::{Clock, Game, Phase, STEP},
};

fn game() -> Game {
    let mut game = Game::new(42);
    game.reset();
    game
}

fn hit_player(game: &mut Game) {
    let p = &game.player.body;
    game.bullets.push(Bullet::new(
        p.x + p.width / 2.0,
        p.y + 1.0,
        BulletOwner::Enemy,
    ));
}

fn enemy_at_player(game: &mut Game, kind: EnemyType) {
    game.formation.enemies.clear();
    let mut enemy = Enemy::new(kind, 400.0, 300.0, 0.0);
    enemy.state = EnemyState::Diving;
    enemy.body.x = game.player.body.x;
    enemy.body.y = game.player.body.y;
    game.formation.enemies.push(enemy);
}

fn enemy_in_crosshairs(kind: EnemyType, state: EnemyState, captured: bool) -> Enemy {
    let mut enemy = Enemy::new(kind, 400.0, 300.0, 0.0);
    enemy.state = state;
    enemy.body.x = 400.0;
    enemy.body.y = 300.0;
    enemy.captured = captured;
    enemy
}

fn shoot_enemy(game: &mut Game) {
    game.bullets
        .push(Bullet::new(412.0, 301.0, BulletOwner::Player));
    game.tick(STEP);
}

#[test]
fn player_starts_with_three_lives() {
    assert_eq!(game().player.lives, 3);
}

#[test]
fn player_can_shoot_one_bullet() {
    let mut g = game();
    g.shoot_pressed = true;
    g.tick(STEP);
    assert_eq!(
        g.bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count(),
        1
    );
}

#[test]
fn player_bullet_count_capped_at_two() {
    let mut g = game();
    for _ in 0..6 {
        g.shoot_pressed = true;
        g.tick(0.001);
    }
    assert_eq!(
        g.bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count(),
        2
    );
}

#[test]
fn player_dies_when_hit_by_enemy_bullet() {
    let mut g = game();
    hit_player(&mut g);
    g.tick(STEP);
    assert_eq!(g.player.lives, 2);
    assert!(!g.player.body.alive);
}

#[test]
fn game_over_when_last_life_lost() {
    let mut g = game();
    g.player.lives = 1;
    hit_player(&mut g);
    g.tick(STEP);
    assert_eq!(g.phase, Phase::GameOver);
}

#[test]
fn stage_clear_triggers_when_all_enemies_dead() {
    let mut g = game();
    for e in &mut g.formation.enemies {
        e.body.alive = false;
    }
    g.tick(STEP);
    assert_eq!(g.phase, Phase::StageClear);
}

#[test]
fn score_increases_when_in_formation_bee_is_killed() {
    let mut g = game();
    g.formation.enemies = vec![enemy_in_crosshairs(
        EnemyType::Bee,
        EnemyState::Formation,
        false,
    )];
    shoot_enemy(&mut g);
    assert!(!g.formation.enemies[0].body.alive);
    assert_eq!(g.score, 50);
}

#[test]
fn player_dies_when_hit_by_diving_enemy() {
    let mut g = game();
    enemy_at_player(&mut g, EnemyType::Bee);
    g.tick(STEP);
    assert_eq!(g.player.lives, 2);
    assert!(!g.player.body.alive);
    assert!(!g.formation.enemies[0].body.alive);
}

#[test]
fn game_over_when_diving_enemy_kills_last_life() {
    let mut g = game();
    g.player.lives = 1;
    enemy_at_player(&mut g, EnemyType::Bee);
    g.tick(STEP);
    assert_eq!(g.phase, Phase::GameOver);
}

#[test]
fn score_gives_diving_bonus() {
    let mut g = game();
    g.formation.enemies = vec![enemy_in_crosshairs(
        EnemyType::Bee,
        EnemyState::Diving,
        false,
    )];
    shoot_enemy(&mut g);
    assert_eq!(g.score, 100);
}

#[test]
fn player_is_invulnerable_after_respawn() {
    let mut g = game();
    g.player.lives = 2;
    g.player.die();
    g.player.respawn_timer = 0.0001;
    g.tick(STEP);
    assert!(g.player.body.alive && g.player.invulnerability > 0.0);
}

#[test]
fn invulnerable_player_cannot_be_killed() {
    let mut g = game();
    g.player.die();
    g.player.respawn_timer = 0.0001;
    g.tick(STEP);
    let lives = g.player.lives;
    hit_player(&mut g);
    g.tick(STEP);
    assert!(g.player.body.alive);
    assert_eq!(g.player.lives, lives);
}

#[test]
fn diving_boss_captures_player() {
    let mut g = game();
    g.player.lives = 2;
    enemy_at_player(&mut g, EnemyType::Boss);
    g.tick(STEP);
    assert!(g.formation.enemies[0].captured);
    assert_eq!(g.player.lives, 1);
    assert!(!g.player.body.alive);
}

#[test]
fn killing_capturing_boss_rescues_dual_fighter() {
    let mut g = game();
    g.formation.enemies = vec![enemy_in_crosshairs(
        EnemyType::Boss,
        EnemyState::Formation,
        true,
    )];
    shoot_enemy(&mut g);
    assert!(!g.formation.enemies[0].body.alive);
    assert!(g.player.dual);
}

#[test]
fn enemies_move_faster_on_higher_levels() {
    let mut low = game();
    let mut high = game();
    high.level = 6;
    low.formation.update(1.0, low.speed_multiplier());
    high.formation.update(1.0, high.speed_multiplier());
    assert!(high.formation.offset > low.formation.offset);
}

#[test]
fn entities_collide_when_overlapping() {
    let p = Body::new(100.0, 100.0, 32.0, 32.0);
    assert!(
        Bullet::new(105.0, 105.0, BulletOwner::Enemy)
            .body
            .collides(&p)
    );
}

#[test]
fn entities_do_not_collide_when_separated() {
    let p = Body::new(100.0, 100.0, 32.0, 32.0);
    assert!(
        !Bullet::new(300.0, 300.0, BulletOwner::Enemy)
            .body
            .collides(&p)
    );
}

#[test]
fn dead_entity_does_not_collide() {
    let mut p = Body::new(100.0, 100.0, 32.0, 32.0);
    p.alive = false;
    assert!(
        !Bullet::new(105.0, 105.0, BulletOwner::Enemy)
            .body
            .collides(&p)
    );
}

#[test]
fn formation_initializes_with_correct_count() {
    let mut formation = Formation::default();
    formation.initialize();
    assert_eq!(formation.enemies.len(), 40);
}

#[test]
fn boss_galaga_placed_in_first_row() {
    let mut formation = Formation::default();
    formation.initialize();
    let bosses: Vec<_> = formation
        .enemies
        .iter()
        .filter(|e| e.kind == EnemyType::Boss)
        .collect();
    assert_eq!(bosses.len(), 2);
    assert!(bosses.iter().all(|e| e.formation_y == 100.0));
}

#[test]
fn pause_freezes_gameplay() {
    let mut g = game();
    g.phase = Phase::Paused;
    let before = g.player.body.x;
    g.player.right = true;
    g.shoot_pressed = true;
    g.tick(STEP);
    assert_eq!(g.player.body.x, before);
    assert!(g.bullets.is_empty());
}

#[test]
fn stage_clear_advances_after_two_and_a_half_seconds() {
    let mut g = game();
    g.formation.enemies.clear();
    g.tick(STEP);
    g.update_stage_clear(2.49);
    assert_eq!(g.phase, Phase::StageClear);
    g.update_stage_clear(0.02);
    assert_eq!(
        (g.phase, g.level, g.formation.alive_count()),
        (Phase::Playing, 2, 40)
    );
}

#[test]
fn dual_fighter_fires_pairs_and_caps_at_four() {
    let mut g = game();
    g.player.dual = true;
    for _ in 0..6 {
        g.shoot_pressed = true;
        g.tick(0.001);
    }
    assert_eq!(
        g.bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count(),
        4
    );
    assert_eq!(
        g.sounds
            .iter()
            .filter(|&&s| s == SoundEffect::Shoot)
            .count(),
        2
    );
}

#[test]
fn dual_fighter_does_not_exceed_cap_when_one_of_four_bullets_expires() {
    let mut g = game();
    g.player.dual = true;
    for x in [100.0, 200.0, 300.0] {
        g.bullets.push(Bullet::new(x, 500.0, BulletOwner::Player));
    }
    g.shoot_pressed = true;
    g.tick(0.001);
    assert_eq!(
        g.bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count(),
        3
    );
    assert!(!g.sounds.contains(&SoundEffect::Shoot));
}

#[test]
fn rescue_grants_thousand_point_bonus_and_dual_is_lost_on_death() {
    let mut g = game();
    g.formation.enemies = vec![enemy_in_crosshairs(
        EnemyType::Boss,
        EnemyState::Formation,
        true,
    )];
    shoot_enemy(&mut g);
    assert_eq!(g.score, 1150);
    assert_eq!(g.high_score, 1150);
    assert!(g.sounds.contains(&SoundEffect::Rescue));
    g.player.die();
    assert!(!g.player.dual);
}

#[test]
fn final_collision_does_not_replace_game_over_with_stage_clear() {
    let mut g = game();
    g.player.lives = 1;
    enemy_at_player(&mut g, EnemyType::Bee);
    g.tick(STEP);
    assert_eq!(g.formation.alive_count(), 0);
    assert_eq!(g.phase, Phase::GameOver);
}

#[test]
fn clock_is_independent_of_refresh_and_caps_stalls() {
    for hz in [60, 120, 144] {
        let mut clock = Clock::default();
        let steps: usize = (0..hz * 16).map(|_| clock.steps(1.0 / f64::from(hz))).sum();
        assert_eq!(steps, 1000);
    }
    let mut clock = Clock::default();
    assert_eq!(clock.steps(10.0), 3);
    clock.clear();
    assert_eq!(clock.steps(0.001), 0);
}

#[test]
fn held_shoot_request_is_consumed_once_across_multiple_steps() {
    let mut g = game();
    g.shoot_pressed = true;
    for _ in 0..3 {
        g.tick(STEP);
    }
    assert_eq!(
        g.bullets
            .iter()
            .filter(|b| b.owner == BulletOwner::Player)
            .count(),
        1
    );
}

#[test]
fn diving_enemy_returns_through_the_top_and_rejoins_formation() {
    let mut enemy = enemy_in_crosshairs(EnemyType::Bee, EnemyState::Formation, false);
    enemy.start_dive(412.0, 564.0);
    for _ in 0..400 {
        enemy.update(STEP, 0.0, 1.0);
        if enemy.state == EnemyState::Returning {
            break;
        }
    }
    assert_eq!(enemy.state, EnemyState::Returning);
    assert_eq!(enemy.body.y, -50.0);
    for _ in 0..300 {
        enemy.update(STEP, 0.0, 1.0);
    }
    assert_eq!(enemy.state, EnemyState::Formation);
}
