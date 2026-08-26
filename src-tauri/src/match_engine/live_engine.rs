/// Live Match Engine — real-time football physics simulation.
///
/// Architecture:
/// - Fixed 60fps physics tick (16.67ms per frame, time-compressed)
/// - Ball: 3D position (x,y,z) with velocity, gravity, friction, spin, bounce
/// - Players: 22 agents with role-based AI, speed/acceleration/inertia
/// - Possession phases: build-up → pass/run → shot → goal/save/restart
/// - Each tick produces a FrameState emitted to the frontend via Tauri events

use serde::{Deserialize, Serialize};

// ─── Coordinate system ─────────────────────────────────────────────────────────
// Pitch is LANDSCAPE:
//   x = 0.0 (left/home goal line) .. 1.0 (right/away goal line)
//   y = 0.0 (top touchline)       .. 1.0 (bottom touchline)
//   z = 0.0 (ground)              .. (metres above pitch, normalised)
// All positions are normalised: 1.0 unit = full pitch length (~105m)

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0 } }
    pub fn dist_to(&self, o: &Vec2) -> f32 {
        let dx = self.x - o.x; let dy = self.y - o.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn normalise(&self) -> Vec2 {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        if len < 0.00001 { Vec2::zero() } else { Vec2::new(self.x / len, self.y / len) }
    }
    pub fn scale(&self, s: f32) -> Vec2 { Vec2::new(self.x * s, self.y * s) }
    pub fn add(&self, o: &Vec2) -> Vec2 { Vec2::new(self.x + o.x, self.y + o.y) }
    pub fn sub(&self, o: &Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
    pub fn len(&self) -> f32 { (self.x * self.x + self.y * self.y).sqrt() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn ground(x: f32, y: f32) -> Self { Self { x, y, z: 0.0 } }
}

// ─── Player attributes (1–20 scale, matches existing DB schema) ───────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttribs {
    pub pace: f32,
    pub acceleration: f32,
    pub finishing: f32,
    pub passing: f32,
    pub heading: f32,
    pub tackling: f32,
    pub dribbling: f32,
    pub reflexes: f32,   // GK shot-stopping
    pub handling: f32,   // GK
    pub positioning: f32,
}

impl Default for PlayerAttribs {
    fn default() -> Self {
        Self {
            pace: 12.0, acceleration: 12.0, finishing: 10.0, passing: 12.0,
            heading: 10.0, tackling: 12.0, dribbling: 10.0,
            reflexes: 12.0, handling: 12.0, positioning: 12.0,
        }
    }
}

impl PlayerAttribs {
    /// Top speed in normalised pitch-lengths per frame (60fps).
    /// pace-20 crosses 105m pitch in ~4s = 240 frames → 1/240 ≈ 0.0042/frame
    pub fn max_speed(&self) -> f32 { (self.pace / 20.0) * 0.0042 }
    pub fn accel_rate(&self) -> f32 { (self.acceleration / 20.0) * 0.15 }
}

// ─── Formation ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role { Goalkeeper, Defender, Midfielder, Attacker }

#[derive(Debug, Clone)]
pub struct FormationSlot {
    pub role: Role,
    pub default_x: f32,
    pub default_y: f32,
}

/// 4-4-2 flat. Home team defends left (x≈0.05), attacks right (x≈0.95).
pub fn build_442() -> Vec<FormationSlot> {
    vec![
        FormationSlot { role: Role::Goalkeeper, default_x: 0.05, default_y: 0.50 },
        FormationSlot { role: Role::Defender,   default_x: 0.20, default_y: 0.18 },
        FormationSlot { role: Role::Defender,   default_x: 0.20, default_y: 0.38 },
        FormationSlot { role: Role::Defender,   default_x: 0.20, default_y: 0.62 },
        FormationSlot { role: Role::Defender,   default_x: 0.20, default_y: 0.82 },
        FormationSlot { role: Role::Midfielder, default_x: 0.42, default_y: 0.18 },
        FormationSlot { role: Role::Midfielder, default_x: 0.42, default_y: 0.38 },
        FormationSlot { role: Role::Midfielder, default_x: 0.42, default_y: 0.62 },
        FormationSlot { role: Role::Midfielder, default_x: 0.42, default_y: 0.82 },
        FormationSlot { role: Role::Attacker,   default_x: 0.68, default_y: 0.38 },
        FormationSlot { role: Role::Attacker,   default_x: 0.68, default_y: 0.62 },
    ]
}

/// Mirror for away team: flip x so they defend right (x≈0.95), attack left (x≈0.05)
pub fn mirror_442(slots: &[FormationSlot]) -> Vec<FormationSlot> {
    slots.iter().map(|s| FormationSlot {
        role: s.role.clone(),
        default_x: 1.0 - s.default_x,
        default_y: s.default_y,
    }).collect()
}

// ─── Ball physics ─────────────────────────────────────────────────────────────

const GRAVITY:        f32 = 0.00018;  // pitch-units/frame²
const GROUND_FRICTION: f32 = 0.980;   // rolling speed retention/frame
const AIR_DRAG:       f32 = 0.995;
const BOUNCE_FACTOR:  f32 = 0.52;     // z-velocity fraction kept on bounce

/// Goal half-width in normalised units (goal posts are 7.32m, pitch 68m wide → ~10.8% of width)
pub const GOAL_HALF_W: f32 = 0.054;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ball {
    pub pos: Vec3,
    pub vel: Vec3,
    pub spin: f32,
    pub last_touched: Side,
}

impl Ball {
    pub fn centre() -> Self {
        Self {
            pos: Vec3::ground(0.50, 0.50),
            vel: Vec3::new(0.0, 0.0, 0.0),
            spin: 0.0,
            last_touched: Side::Home,
        }
    }

    pub fn tick(&mut self) {
        if self.pos.z > 0.0 || self.vel.z > 0.0 {
            // Airborne
            self.vel.z -= GRAVITY;
            self.vel.x *= AIR_DRAG;
            self.vel.y *= AIR_DRAG;
            self.spin  *= 0.97;

            self.pos.x += self.vel.x;
            self.pos.y += self.vel.y;
            self.pos.z += self.vel.z;

            if self.pos.z <= 0.0 {
                self.pos.z = 0.0;
                if self.vel.z.abs() > 0.0003 {
                    self.vel.z = -self.vel.z * BOUNCE_FACTOR;
                } else {
                    self.vel.z = 0.0;
                }
            }
        } else {
            // Rolling
            self.vel.x *= GROUND_FRICTION;
            self.vel.y *= GROUND_FRICTION;
            self.spin  *= 0.96;
            self.pos.x += self.vel.x;
            self.pos.y += self.vel.y;
        }
    }

    pub fn kick(&mut self, target: &Vec2, power: f32, lift: f32) {
        let dir = Vec2::new(target.x - self.pos.x, target.y - self.pos.y).normalise();
        let speed = power * 0.020;
        self.vel.x = dir.x * speed;
        self.vel.y = dir.y * speed;
        self.vel.z = lift * 0.016;
    }

    pub fn xy(&self) -> Vec2 { Vec2::new(self.pos.x, self.pos.y) }

    pub fn in_goal_home(&self) -> bool {
        self.pos.x < 0.008 && (self.pos.y - 0.5).abs() < GOAL_HALF_W
    }
    pub fn in_goal_away(&self) -> bool {
        self.pos.x > 0.992 && (self.pos.y - 0.5).abs() < GOAL_HALF_W
    }
}

// ─── Teams ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Side { Home, Away }

impl Side {
    pub fn flip(self) -> Side { if self == Side::Home { Side::Away } else { Side::Home } }
    pub fn own_goal_x(self) -> f32 { if self == Side::Home { 0.0 } else { 1.0 } }
    pub fn attack_goal_x(self) -> f32 { if self == Side::Home { 1.0 } else { 0.0 } }
}

// ─── Player agent ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Pose { Stand, Walk, Run, Kick, Slide, Fall, Header }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub side: Side,
    pub role: Role,
    pub slot: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub speed: f32,
    pub dir: f32,          // radians, 0=right
    pub has_ball: bool,
    pub stamina: f32,      // 1.0 = fresh
    pub attribs: PlayerAttribs,
    pub default_x: f32,
    pub default_y: f32,
    pub skin_tone: u8,     // 0-3
    pub pose: Pose,
    pub pose_timer: u32,   // frames since pose started
    pub name_short: String,
}

// ─── Events ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    KickOff, Pass, Shot, Goal, Save, Tackle, Miss,
    ThrowIn, GoalKick, Corner, FreeKick, HalfTime, FullTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub tick: u32,
    pub minute: u32,
    pub second: u32,
    pub kind: EventKind,
    pub side: Side,
    pub ball_x: f32,
    pub ball_y: f32,
    pub player_id: Option<String>,
    pub description: String,
}

// ─── Frame snapshot (sent to frontend each tick) ──────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameState {
    pub tick: u32,
    pub minute: u32,
    pub second: u32,
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_z: f32,       // normalised 0..1 height
    pub ball_speed: f32,
    pub ball_frame: u8,    // sprite frame index 0-3
    pub players: Vec<PlayerSnap>,
    pub home_score: u32,
    pub away_score: u32,
    pub possession: Side,
    pub home_poss_pct: f32,
    pub phase: Phase,
    pub event: Option<MatchEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnap {
    pub id: String,
    pub side: Side,
    pub x: f32,
    pub y: f32,
    pub dir_deg: f32,
    pub pose: Pose,
    pub pose_timer: u32,
    pub has_ball: bool,
    pub skin_tone: u8,
    pub is_gk: bool,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Phase { FirstHalf, HalfTime, SecondHalf, FullTime }

// ─── Stats ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchStats {
    pub home_shots: u32,
    pub away_shots: u32,
    pub home_on_target: u32,
    pub away_on_target: u32,
    pub home_corners: u32,
    pub away_corners: u32,
    pub home_poss_frames: u32,
    pub away_poss_frames: u32,
}

impl MatchStats {
    pub fn poss_pct(&self, side: Side) -> f32 {
        let total = self.home_poss_frames + self.away_poss_frames;
        if total == 0 { return 50.0; }
        let f = if side == Side::Home { self.home_poss_frames } else { self.away_poss_frames };
        (f as f32 / total as f32) * 100.0
    }
}

// ─── Deterministic RNG ────────────────────────────────────────────────────────

pub struct Rng { s: u64 }
impl Rng {
    pub fn new(seed: u64) -> Self { Self { s: seed ^ 0xdeadbeef12345678 } }
    fn next(&mut self) -> u64 {
        self.s ^= self.s << 13; self.s ^= self.s >> 7; self.s ^= self.s << 17; self.s
    }
    pub fn f(&mut self) -> f32 { (self.next() & 0xFFFFFF) as f32 / 16777216.0 }
    pub fn range(&mut self, lo: f32, hi: f32) -> f32 { lo + self.f() * (hi - lo) }
    pub fn ri(&mut self, lo: i32, hi: i32) -> i32 { lo + (self.next() % (hi - lo).max(1) as u64) as i32 }
    pub fn chance(&mut self, p: f32) -> bool { self.f() < p }
}

// ─── Possession state machine ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Poss {
    KickOff,
    BuildUp  { carrier: usize, held: u32 },
    InFlight { from: usize, to: usize, flight: u32 },
    ShotFlight{ shooter: usize, flight: u32, on_target: bool },
    GoalKick { side: Side },
    Corner   { side: Side },
    ThrowIn  { side: Side, touchline_y: f32 },
    FreeKick { side: Side },
    Pause    { left: u32 },
}

// ─── Time constants ───────────────────────────────────────────────────────────
// 60fps × time-compression: 1 real second = 30 game seconds
// Full match: 90 min × 60s × (60fps / 30 gs/rs) = 10,800 ticks per half
// = 21,600 ticks total ≈ 360 real seconds ≈ 6 real minutes

const FPS: u32 = 60;
const GAME_S_PER_REAL_S: u32 = 30;
const TICKS_PER_GAME_S: u32 = FPS / GAME_S_PER_REAL_S; // = 2 ticks per game-second
const HALF_TICKS: u32 = 45 * 60 * TICKS_PER_GAME_S;   // 5400 ticks per half

// ─── Main engine ──────────────────────────────────────────────────────────────

pub struct LiveMatchEngine {
    pub match_id: String,
    pub home_players: Vec<Player>,
    pub away_players: Vec<Player>,
    pub ball: Ball,
    pub home_score: u32,
    pub away_score: u32,
    pub tick: u32,
    pub phase: Phase,
    pub stats: MatchStats,

    possession: Side,
    poss: Poss,
    rng: Rng,
    ball_frame: u8,
    ball_frame_timer: u32,
    ball_last_xy: Vec2,

    home_formation: Vec<FormationSlot>,
    away_formation: Vec<FormationSlot>,
}

impl LiveMatchEngine {
    pub fn new(
        match_id: String,
        home_attribs: Vec<PlayerAttribs>,
        away_attribs: Vec<PlayerAttribs>,
        home_name: &str,
        away_name: &str,
        seed: u64,
    ) -> Self {
        let mut rng = Rng::new(seed);
        let home_formation = build_442();
        let away_formation = mirror_442(&home_formation);

        let home_players = build_squad(&home_attribs, &home_formation, Side::Home, home_name, &mut rng);
        let away_players = build_squad(&away_attribs, &away_formation, Side::Away, away_name, &mut rng);

        Self {
            match_id,
            home_players,
            away_players,
            ball: Ball::centre(),
            home_score: 0,
            away_score: 0,
            tick: 0,
            phase: Phase::FirstHalf,
            stats: MatchStats::default(),
            possession: Side::Home,
            poss: Poss::KickOff,
            rng,
            ball_frame: 0,
            ball_frame_timer: 0,
            ball_last_xy: Vec2::new(0.5, 0.5),
            home_formation,
            away_formation,
        }
    }

    /// Advance one physics frame. Returns the complete frame snapshot for the frontend.
    pub fn step(&mut self) -> FrameState {
        let mut ev: Option<MatchEvent> = None;

        // ── Phase transitions ────────────────────────────────────────────────
        match &self.phase {
            Phase::FirstHalf => {
                ev = self.game_tick();
                if self.tick >= HALF_TICKS {
                    self.phase = Phase::HalfTime;
                    self.poss = Poss::Pause { left: 120 };
                    ev = Some(self.make_ev(EventKind::HalfTime, "Half Time".into(), None));
                }
            }
            Phase::HalfTime => {
                if let Poss::Pause { left } = &mut self.poss {
                    if *left > 0 { *left -= 1; } else {
                        self.phase = Phase::SecondHalf;
                        self.possession = self.possession.flip();
                        self.ball = Ball::centre();
                        self.poss = Poss::KickOff;
                        ev = Some(self.make_ev(EventKind::KickOff, "Second Half".into(), None));
                    }
                }
            }
            Phase::SecondHalf => {
                ev = self.game_tick();
                if self.tick >= HALF_TICKS * 2 {
                    self.phase = Phase::FullTime;
                    ev = Some(self.make_ev(EventKind::FullTime, "Full Time".into(), None));
                }
            }
            Phase::FullTime => {}
        }

        // ── Ball physics ─────────────────────────────────────────────────────
        if self.phase != Phase::FullTime && self.phase != Phase::HalfTime {
            self.ball.tick();
        }

        // ── Ball sprite frame ────────────────────────────────────────────────
        let bxy = self.ball.xy();
        let bdx = bxy.x - self.ball_last_xy.x;
        let bdy = bxy.y - self.ball_last_xy.y;
        let bspeed = (bdx * bdx + bdy * bdy).sqrt();
        self.ball_last_xy = bxy.clone();

        let fi = if bspeed > 0.003 { 3u32 } else if bspeed > 0.001 { 7u32 } else { 0u32 };
        if fi > 0 {
            self.ball_frame_timer += 1;
            if self.ball_frame_timer >= fi {
                self.ball_frame_timer = 0;
                self.ball_frame = (self.ball_frame + 1) % 4;
            }
        }

        // ── Possession tracking ──────────────────────────────────────────────
        match self.possession {
            Side::Home => self.stats.home_poss_frames += 1,
            Side::Away => self.stats.away_poss_frames += 1,
        }

        // ── Animate player pose timers ───────────────────────────────────────
        for p in self.home_players.iter_mut().chain(self.away_players.iter_mut()) {
            p.pose_timer += 1;
            match p.pose {
                Pose::Kick   if p.pose_timer > 24 => { p.pose = Pose::Run; p.pose_timer = 0; }
                Pose::Slide  if p.pose_timer > 36 => { p.pose = Pose::Walk; p.pose_timer = 0; }
                Pose::Fall   if p.pose_timer > 48 => { p.pose = Pose::Stand; p.pose_timer = 0; }
                Pose::Header if p.pose_timer > 30 => { p.pose = Pose::Walk; p.pose_timer = 0; }
                _ => {}
            }
        }

        self.tick += 1;
        let (minute, second) = self.game_time();
        let ball_z_norm = (self.ball.pos.z / 0.22).min(1.0).max(0.0);

        FrameState {
            tick: self.tick,
            minute,
            second,
            ball_x: self.ball.pos.x,
            ball_y: self.ball.pos.y,
            ball_z: ball_z_norm,
            ball_speed: bspeed,
            ball_frame: self.ball_frame,
            players: self.snap_players(),
            home_score: self.home_score,
            away_score: self.away_score,
            possession: self.possession,
            home_poss_pct: self.stats.poss_pct(Side::Home),
            phase: self.phase.clone(),
            event: ev,
        }
    }

    pub fn is_finished(&self) -> bool { self.phase == Phase::FullTime }

    // ── Internal ─────────────────────────────────────────────────────────────

    fn game_time(&self) -> (u32, u32) {
        let gs = self.tick / TICKS_PER_GAME_S;
        (gs / 60, gs % 60)
    }

    fn make_ev(&self, kind: EventKind, desc: String, player_id: Option<String>) -> MatchEvent {
        let (minute, second) = self.game_time();
        MatchEvent {
            tick: self.tick, minute, second,
            kind, side: self.possession,
            ball_x: self.ball.pos.x, ball_y: self.ball.pos.y,
            player_id, description: desc,
        }
    }

    fn game_tick(&mut self) -> Option<MatchEvent> {
        match self.poss.clone() {

            Poss::KickOff => {
                self.ball = Ball::centre();
                let ci = self.first_attacker(self.possession);
                self.squad_mut(self.possession)[ci].has_ball = true;
                self.squad_mut(self.possession)[ci].pos = Vec2::new(0.50, 0.50);
                self.clear_ball_except(self.possession, ci);
                self.poss = Poss::BuildUp { carrier: ci, held: 0 };
                Some(self.make_ev(EventKind::KickOff, "Kick Off!".into(), None))
            }

            Poss::BuildUp { carrier, held } => {
                let side = self.possession;
                self.ai_move_all(side, carrier);
                // Ball is kept on carrier inside ai_move_all

                // Check tackle
                if self.try_tackle(side, carrier) {
                    let opp = side.flip();
                    let ni = self.nearest_to_ball(opp);
                    self.clear_all_ball();
                    self.squad_mut(opp)[ni].has_ball = true;
                    self.possession = opp;
                    self.poss = Poss::BuildUp { carrier: ni, held: 0 };
                    return Some(self.make_ev(EventKind::Tackle, "Tackle!".into(), None));
                }

                // Out of bounds
                if let Some(next) = self.check_bounds(side) {
                    self.poss = next;
                    return None;
                }

                // Goal check
                if let Some(ev) = self.check_goal() { return Some(ev); }

                // Decide action
                let px = self.squad(side)[carrier].pos.x;
                let attacking_x = if side == Side::Home { px } else { 1.0 - px };
                let hold_limit = self.rng.ri(18, 50) as u32;

                if held >= hold_limit {
                    let shoot_prob = if attacking_x > 0.72 { 0.38 } else { 0.08 };
                    if self.rng.chance(shoot_prob) {
                        let fin = self.squad(side)[carrier].attribs.finishing;
                        let on_target = self.rng.chance(0.30 + (fin / 20.0) * 0.50);
                        let goal_x = side.attack_goal_x();
                        let goal_y = if on_target {
                            0.50 + self.rng.range(-GOAL_HALF_W * 0.85, GOAL_HALF_W * 0.85)
                        } else {
                            0.50 + self.rng.range(-0.22, 0.22)
                        };
                        let tgt = Vec2::new(goal_x, goal_y);
                        let pwr = self.rng.range(0.65, 0.98);
                        let lft = if on_target { self.rng.range(0.0, 0.30) } else { self.rng.range(0.25, 0.70) };
                        self.ball.kick(&tgt, pwr, lft);
                        self.ball.last_touched = side;
                        self.squad_mut(side)[carrier].has_ball = false;
                        self.squad_mut(side)[carrier].pose = Pose::Kick;
                        self.squad_mut(side)[carrier].pose_timer = 0;
                        self.add_shot_stat(side, on_target);
                        self.poss = Poss::ShotFlight { shooter: carrier, flight: 0, on_target };
                        return Some(self.make_ev(EventKind::Shot, "Shot!".into(), Some(self.squad(side)[carrier].id.clone())));
                    } else if let Some(ti) = self.pick_pass_target(side, carrier) {
                        let tpos = self.squad(side)[ti].pos.clone();
                        let pwr = self.rng.range(0.45, 0.85);
                        self.ball.kick(&tpos, pwr, 0.0);
                        self.ball.last_touched = side;
                        self.squad_mut(side)[carrier].has_ball = false;
                        self.squad_mut(side)[carrier].pose = Pose::Kick;
                        self.squad_mut(side)[carrier].pose_timer = 0;
                        self.poss = Poss::InFlight { from: carrier, to: ti, flight: 0 };
                        return Some(self.make_ev(EventKind::Pass, String::new(), None));
                    }
                }

                self.poss = Poss::BuildUp { carrier, held: held + 1 };
                None
            }

            Poss::InFlight { from, to, flight } => {
                let side = self.possession;
                self.ai_move_all(side, from);

                let bxy = self.ball.xy();

                // Intercept check
                let opp = side.flip();
                if let Some(ii) = self.intercept_check(opp, &bxy) {
                    self.clear_all_ball();
                    self.squad_mut(opp)[ii].has_ball = true;
                    self.possession = opp;
                    self.poss = Poss::BuildUp { carrier: ii, held: 0 };
                    return None;
                }

                let recip_pos = self.squad(side)[to].pos.clone();
                let dist = bxy.dist_to(&recip_pos);

                if dist < 0.045 || flight > 100 {
                    let pass_skill = self.squad(side)[from].attribs.passing;
                    if self.rng.chance(0.18 + (pass_skill / 20.0) * 0.75) {
                        // Received
                        self.clear_all_ball();
                        self.squad_mut(side)[to].has_ball = true;
                        self.ball.pos = Vec3::ground(recip_pos.x, recip_pos.y);
                        self.ball.vel = Vec3::new(0.0, 0.0, 0.0);
                        self.poss = Poss::BuildUp { carrier: to, held: 0 };
                    } else {
                        // Miscontrol — loose ball
                        let (tid, ti) = self.nearest_either_team();
                        if !tid.is_empty() {
                            let ts = self.id_to_side(&tid);
                            self.clear_all_ball();
                            if let Some(p) = self.squad_mut_by_id(ts, &tid) { p.has_ball = true; }
                            self.possession = ts;
                            self.poss = Poss::BuildUp { carrier: ti, held: 0 };
                        }
                    }
                } else {
                    self.poss = Poss::InFlight { from, to, flight: flight + 1 };
                }
                None
            }

            Poss::ShotFlight { shooter, flight, on_target } => {
                let side = self.possession;

                if let Some(ev) = self.check_goal() {
                    self.poss = Poss::KickOff;
                    return Some(ev);
                }

                let past = match side {
                    Side::Home => self.ball.pos.x > 0.985,
                    Side::Away => self.ball.pos.x < 0.015,
                };

                if past || flight > 140 {
                    if on_target {
                        let gk_ref = self.squad(side.flip())[0].attribs.reflexes;
                        let save_p = 0.28 + (gk_ref / 20.0) * 0.58;
                        let saved = self.rng.chance(save_p);
                        // Place ball for GK
                        let gk_x = if side == Side::Home { 0.94 } else { 0.06 };
                        self.ball.pos = Vec3::ground(gk_x, 0.50);
                        self.ball.vel = Vec3::new(0.0, 0.0, 0.0);
                        self.possession = side.flip();
                        self.poss = Poss::GoalKick { side: side.flip() };
                        if saved {
                            return Some(self.make_ev(EventKind::Save, "Saved!".into(),
                                Some(self.squad(side.flip())[0].id.clone())));
                        }
                    } else {
                        // Miss — corner or goal kick
                        let over_end = match side {
                            Side::Home => self.ball.pos.x > 0.95,
                            Side::Away => self.ball.pos.x < 0.05,
                        };
                        if over_end && self.rng.chance(0.45) {
                            self.stats.home_corners += if side == Side::Away { 1 } else { 0 };
                            self.stats.away_corners += if side == Side::Home { 1 } else { 0 };
                            self.poss = Poss::Corner { side: side.flip() };
                            return Some(self.make_ev(EventKind::Miss, "Corner!".into(), None));
                        } else {
                            self.poss = Poss::GoalKick { side: side.flip() };
                            return Some(self.make_ev(EventKind::Miss, "Over the bar!".into(), None));
                        }
                    }
                } else {
                    self.poss = Poss::ShotFlight { shooter, flight: flight + 1, on_target };
                }
                None
            }

            Poss::GoalKick { side } => {
                let gk_x = if side == Side::Home { 0.06 } else { 0.94 };
                self.ball.pos = Vec3::ground(gk_x, 0.50);
                self.ball.vel = Vec3::new(0.0, 0.0, 0.0);
                self.possession = side;
                self.clear_all_ball();
                self.squad_mut(side)[0].has_ball = true;
                self.poss = Poss::BuildUp { carrier: 0, held: 0 };
                Some(self.make_ev(EventKind::GoalKick, "Goal Kick".into(), None))
            }

            Poss::Corner { side } => {
                let cx = if side == Side::Home { 0.995 } else { 0.005 };
                let cy = if self.rng.chance(0.5) { 0.02 } else { 0.98 };
                self.ball.pos = Vec3::ground(cx, cy);
                let ty = 0.50 + self.rng.range(-0.18, 0.18);
                let tx = if side == Side::Home { 0.87 } else { 0.13 };
                let tgt = Vec2::new(tx, ty);
                self.ball.kick(&tgt, 0.72, 0.45);
                self.possession = side;
                self.clear_all_ball();
                let fi = self.first_attacker(side);
                self.squad_mut(side)[fi].has_ball = true;
                self.poss = Poss::BuildUp { carrier: fi, held: 0 };
                Some(self.make_ev(EventKind::Corner, "Corner!".into(), None))
            }

            Poss::ThrowIn { side, touchline_y } => {
                let tx = self.ball.pos.x.clamp(0.05, 0.95);
                self.ball.pos = Vec3::ground(tx, touchline_y);
                let tgt = Vec2::new(tx, 0.50);
                self.ball.kick(&tgt, 0.28, 0.0);
                self.possession = side;
                self.clear_all_ball();
                let ni = self.nearest_to_ball(side);
                self.squad_mut(side)[ni].has_ball = true;
                self.poss = Poss::BuildUp { carrier: ni, held: 0 };
                Some(self.make_ev(EventKind::ThrowIn, "Throw In".into(), None))
            }

            Poss::FreeKick { side } => {
                let ni = self.nearest_to_ball(side);
                self.possession = side;
                self.clear_all_ball();
                self.squad_mut(side)[ni].has_ball = true;
                self.poss = Poss::BuildUp { carrier: ni, held: 0 };
                Some(self.make_ev(EventKind::FreeKick, "Free Kick".into(), None))
            }

            Poss::Pause { .. } => None,
        }
    }

    // ── AI movement ───────────────────────────────────────────────────────────

    fn ai_move_all(&mut self, attacking: Side, carrier: usize) {
        let ball_xy = self.ball.xy();
        let possession = self.possession;

        // Snapshot formation defaults to avoid borrow conflicts
        let home_slots: Vec<(f32, f32, Role)> = self.home_formation.iter()
            .map(|s| (s.default_x, s.default_y, s.role.clone())).collect();
        let away_slots: Vec<(f32, f32, Role)> = self.away_formation.iter()
            .map(|s| (s.default_x, s.default_y, s.role.clone())).collect();

        for side in [Side::Home, Side::Away] {
            let slots = if side == Side::Home { &home_slots } else { &away_slots };
            let squad = if side == Side::Home { &mut self.home_players } else { &mut self.away_players };
            let carrier_for_side = if side == possession { Some(carrier) } else { None };
            for (i, player) in squad.iter_mut().enumerate() {
                let ms = player.attribs.max_speed() * player.stamina;
                let ar = player.attribs.accel_rate();

                if player.has_ball {
                    // Carrier dribbles toward goal
                    let goal_x = if side == Side::Home { 1.0f32 } else { 0.0f32 };
                    let ddx = goal_x - player.pos.x;
                    let ddy = 0.5f32 - player.pos.y; // drift toward centre-y
                    let dist = (ddx * ddx + ddy * ddy).sqrt();
                    if dist > 0.01 {
                        let drib_speed = ms * 0.70; // carrier moves at 70% max speed
                        player.speed += (drib_speed - player.speed) * ar;
                        player.speed = player.speed.min(drib_speed);
                        player.vel = Vec2::new(ddx / dist * player.speed, ddy / dist * player.speed);
                        player.dir = ddy.atan2(ddx);
                        player.pose = Pose::Run;
                    }
                    player.pos.x = (player.pos.x + player.vel.x).clamp(0.01, 0.99);
                    player.pos.y = (player.pos.y + player.vel.y).clamp(0.01, 0.99);
                    player.stamina = (player.stamina - player.speed * 0.0004).max(0.55);
                    continue;
                }

                let (dx, dy, ref role) = slots[i];
                let tgt = ai_target(player, role, dx, dy, &ball_xy, possession, side);
                let ddx = tgt.x - player.pos.x;
                let ddy = tgt.y - player.pos.y;
                let dist = (ddx * ddx + ddy * ddy).sqrt();
                if dist > 0.002 {
                    let desired = (dist * 5.0).min(ms);
                    player.speed += (desired - player.speed) * ar;
                    player.speed = player.speed.min(ms);
                    player.vel = Vec2::new(ddx / dist * player.speed, ddy / dist * player.speed);
                    player.dir = ddy.atan2(ddx);
                    player.pose = if player.speed > ms * 0.55 { Pose::Run } else { Pose::Walk };
                } else {
                    player.speed *= 0.80;
                    player.vel = Vec2::zero();
                    if player.pose == Pose::Walk || player.pose == Pose::Run {
                        player.pose = Pose::Stand;
                    }
                }
                player.pos.x = (player.pos.x + player.vel.x).clamp(0.01, 0.99);
                player.pos.y = (player.pos.y + player.vel.y).clamp(0.01, 0.99);
                player.stamina = (player.stamina - player.speed * 0.0004).max(0.55);
            }
        }

        // Keep ball glued to carrier position
        if let Some(carrier_idx) = Some(carrier) {
            let cp = self.squad(possession)[carrier_idx].pos.clone();
            self.ball.pos.x = cp.x;
            self.ball.pos.y = cp.y;
            self.ball.pos.z = 0.0;
            self.ball.vel = Vec3::new(0.0, 0.0, 0.0);
        }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn check_goal(&mut self) -> Option<MatchEvent> {
        if self.ball.in_goal_away() {
            self.home_score += 1;
            let ev = self.make_ev(EventKind::Goal, "GOAL!".into(), None);
            self.ball = Ball::centre();
            self.possession = Side::Away;
            self.clear_all_ball();
            return Some(ev);
        }
        if self.ball.in_goal_home() {
            self.away_score += 1;
            let ev = self.make_ev(EventKind::Goal, "GOAL!".into(), None);
            self.ball = Ball::centre();
            self.possession = Side::Home;
            self.clear_all_ball();
            return Some(ev);
        }
        None
    }

    fn check_bounds(&mut self, side: Side) -> Option<Poss> {
        let by = self.ball.pos.y;
        let bx = self.ball.pos.x;
        if by < 0.0 || by > 1.0 {
            let tl = if by < 0.0 { 0.01 } else { 0.99 };
            self.ball.pos.y = tl;
            return Some(Poss::ThrowIn { side: side.flip(), touchline_y: tl });
        }
        if bx < 0.0 || bx > 1.0 {
            let defending = if bx < 0.0 { Side::Home } else { Side::Away };
            return Some(Poss::GoalKick { side: defending });
        }
        None
    }

    fn try_tackle(&mut self, attacking: Side, carrier: usize) -> bool {
        let cp = self.squad(attacking)[carrier].pos.clone();
        let drib = self.squad(attacking)[carrier].attribs.dribbling;
        let def_side = attacking.flip();
        let def_snap: Vec<(Vec2, f32, Role)> = self.squad(def_side).iter()
            .map(|p| (p.pos.clone(), p.attribs.tackling, p.role.clone())).collect();
        for (pos, tack, role) in &def_snap {
            if *role == Role::Goalkeeper { continue; }
            if pos.dist_to(&cp) < 0.038 {
                let net = (tack / 20.0) * 0.32 - (drib / 20.0) * 0.18;
                if self.rng.chance(net.clamp(0.04, 0.48)) { return true; }
            }
        }
        false
    }

    fn intercept_check(&mut self, def_side: Side, ball: &Vec2) -> Option<usize> {
        let snap: Vec<(Vec2, Role)> = self.squad(def_side).iter()
            .map(|p| (p.pos.clone(), p.role.clone())).collect();
        for (i, (pos, role)) in snap.iter().enumerate() {
            if *role == Role::Goalkeeper { continue; }
            if pos.dist_to(ball) < 0.048 && self.rng.chance(0.22) { return Some(i); }
        }
        None
    }

    fn pick_pass_target(&mut self, side: Side, from: usize) -> Option<usize> {
        let fp = self.squad(side)[from].pos.clone();
        let fx = fp.x;
        let snap: Vec<(Vec2, Role)> = self.squad(side).iter()
            .map(|p| (p.pos.clone(), p.role.clone())).collect();
        let mut cands: Vec<(usize, f32)> = snap.iter().enumerate()
            .filter(|(i, (_, r))| *i != from && *r != Role::Goalkeeper)
            .map(|(i, (pos, _))| {
                let ahead = if side == Side::Home { pos.x - fx } else { fx - pos.x };
                let dist  = pos.dist_to(&fp);
                (i, ahead * 1.8 - dist * 0.5)
            })
            .filter(|(_, s)| *s > -0.4)
            .collect();
        cands.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        cands.first().map(|(i, _)| *i)
    }

    fn first_attacker(&self, side: Side) -> usize {
        self.squad(side).iter().position(|p| p.role == Role::Attacker).unwrap_or(9)
    }

    fn nearest_to_ball(&self, side: Side) -> usize {
        let bxy = self.ball.xy();
        self.squad(side).iter().enumerate()
            .min_by(|(_, a), (_, b)|
                a.pos.dist_to(&bxy).partial_cmp(&b.pos.dist_to(&bxy)).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0)
    }

    fn nearest_either_team(&self) -> (String, usize) {
        let bxy = self.ball.xy();
        let mut best = f32::MAX;
        let mut out = (String::new(), 0usize);
        for (i, p) in self.home_players.iter().enumerate() {
            let d = p.pos.dist_to(&bxy);
            if d < best { best = d; out = (p.id.clone(), i); }
        }
        for (i, p) in self.away_players.iter().enumerate() {
            let d = p.pos.dist_to(&bxy);
            if d < best { best = d; out = (p.id.clone(), i); }
        }
        out
    }

    fn id_to_side(&self, id: &str) -> Side {
        if self.home_players.iter().any(|p| p.id == id) { Side::Home } else { Side::Away }
    }

    fn clear_all_ball(&mut self) {
        for p in self.home_players.iter_mut().chain(self.away_players.iter_mut()) { p.has_ball = false; }
    }

    fn clear_ball_except(&mut self, side: Side, keep: usize) {
        let opp = side.flip();
        for p in self.squad_mut(opp).iter_mut() { p.has_ball = false; }
        let sq = self.squad_mut(side);
        for (i, p) in sq.iter_mut().enumerate() { if i != keep { p.has_ball = false; } }
    }

    fn squad(&self, side: Side) -> &Vec<Player> {
        if side == Side::Home { &self.home_players } else { &self.away_players }
    }

    fn squad_mut(&mut self, side: Side) -> &mut Vec<Player> {
        if side == Side::Home { &mut self.home_players } else { &mut self.away_players }
    }

    fn squad_mut_by_id(&mut self, side: Side, id: &str) -> Option<&mut Player> {
        let sq = if side == Side::Home { &mut self.home_players } else { &mut self.away_players };
        sq.iter_mut().find(|p| p.id == id)
    }

    fn add_shot_stat(&mut self, side: Side, on_target: bool) {
        match side {
            Side::Home => { self.stats.home_shots += 1; if on_target { self.stats.home_on_target += 1; } }
            Side::Away => { self.stats.away_shots += 1; if on_target { self.stats.away_on_target += 1; } }
        }
    }

    fn snap_players(&self) -> Vec<PlayerSnap> {
        self.home_players.iter().chain(self.away_players.iter())
            .map(|p| PlayerSnap {
                id: p.id.clone(),
                side: p.side,
                x: p.pos.x,
                y: p.pos.y,
                dir_deg: p.dir.to_degrees().rem_euclid(360.0),
                pose: p.pose.clone(),
                pose_timer: p.pose_timer,
                has_ball: p.has_ball,
                skin_tone: p.skin_tone,
                is_gk: p.role == Role::Goalkeeper,
                name: p.name_short.clone(),
            })
            .collect()
    }
}

// ─── Squad builder ────────────────────────────────────────────────────────────

fn build_squad(
    attribs: &[PlayerAttribs],
    formation: &[FormationSlot],
    side: Side,
    _team_name: &str,
    rng: &mut Rng,
) -> Vec<Player> {
    let default_a = PlayerAttribs::default();
    (0..11).map(|i| {
        let slot  = &formation[i];
        let a     = attribs.get(i).cloned().unwrap_or(default_a.clone());
        let prefix = if side == Side::Home { "h" } else { "a" };
        let short  = match i {
            0 => "GK".to_string(),
            _ => format!("{}{}", prefix.to_uppercase(), i + 1),
        };
        Player {
            id:        format!("{}_{}", prefix, i),
            name:      format!("Player {}", i + 1),
            name_short: short,
            side,
            role:      slot.role.clone(),
            slot:      i,
            pos:       Vec2::new(slot.default_x, slot.default_y),
            vel:       Vec2::zero(),
            speed:     0.0,
            dir:       if side == Side::Home { 0.0 } else { std::f32::consts::PI },
            has_ball:  false,
            stamina:   1.0,
            attribs:   a,
            default_x: slot.default_x,
            default_y: slot.default_y,
            skin_tone: rng.ri(0, 4) as u8,
            pose:      Pose::Stand,
            pose_timer: 0,
        }
    }).collect()
}

// ─── AI target position ────────────────────────────────────────────────────────

fn ai_target(
    player: &Player,
    role: &Role,
    default_x: f32,
    default_y: f32,
    ball: &Vec2,
    possession: Side,
    side: Side,
) -> Vec2 {
    let we_attack = possession == side;
    // Normalise so "low x = own goal, high x = opponent goal" for both teams
    let (eff_ball_x, eff_def_x) = if side == Side::Away {
        (1.0 - ball.x, 1.0 - default_x)
    } else {
        (ball.x, default_x)
    };
    let ball_y = ball.y;

    let (tx, ty): (f32, f32) = match role {
        Role::Goalkeeper => {
            let gy = (ball_y - 0.5).clamp(-0.10, 0.10) + 0.5;
            (0.05, gy)
        }
        Role::Defender => {
            if we_attack {
                ((eff_def_x + 0.07).min(0.42), (ball_y - 0.5).clamp(-0.22, 0.22) + 0.5)
            } else {
                ((eff_def_x - 0.04).max(0.10), default_y * 0.55 + 0.5 * 0.45)
            }
        }
        Role::Midfielder => {
            let tx = if we_attack {
                (eff_ball_x * 0.50 + eff_def_x * 0.50).min(0.72)
            } else {
                (eff_ball_x * 0.32 + eff_def_x * 0.68).min(0.58)
            };
            (tx, (ball_y - 0.5).clamp(-0.20, 0.20) + 0.5)
        }
        Role::Attacker => {
            if we_attack {
                ((eff_ball_x * 0.35 + eff_def_x * 0.65).max(0.62), default_y)
            } else {
                (0.52, default_y)
            }
        }
    };

    let out_x = if side == Side::Away { 1.0 - tx } else { tx };
    Vec2::new(out_x.clamp(0.01, 0.99), ty.clamp(0.02, 0.98))
}
